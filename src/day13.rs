use claw_machine::Machine;
use regex::Regex;
use std::fs;

//Part 1 solution notes:
//it costs 3 tokens to push the A button and 1 token to push the B button.
//Each button moves right and forward.

//For each machine we want to solve the following system of equations for

// A_x: the movement along the x-axis that pressing A yields
// Prize_x: the x corrdinate of the prize
// A_num: number of presses of A

// A_x * A_num + B_x * B_num = Prize_x
// A_y * A_num + B_y * B_num = Prize_y
//Solve for A_num and B_num such that z = 3* A_num + 1* B_num is minimized.
//We are also given 0<= A_num, B_num <=100.
//We also insist that A_num and B_num are integers.
//If these equations are linearly-independent, then there is at most one solution in the integers.
//If these equations are linearly-dependent (that is they are essentially the same equation; Say the first one
//is just the second one times 2 for example), then we have a free variable.

//I think that in general (if we had more equations), we solve this by using Gaussian Elimination.
//For part 1: All we are doing is finding where the lines these equations represent intersect, and
//if that intersection is in the natural numbers (and A_num, B_num <=100).

//Note that there are machines for which no solution exists.

pub mod claw_machine {
    #[derive(Debug)]
    pub struct Machine {
        a_x: u64,
        a_y: u64,
        b_x: u64,
        b_y: u64,
        p_x: u64,
        p_y: u64,
    }

    impl Machine {
        pub fn new(a_x: u64, a_y: u64, b_x: u64, b_y: u64, p_x: u64, p_y: u64) -> Machine {
            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                p_x,
                p_y,
            }
        }

        ///Returns the minimal number of tokens needed to solve the machine. If there is no solution, returns None.
        pub fn price(&self, part_2: bool) -> Option<u64> {
            //The price of the machine is = 3* A_num + 1* B_num
            self.solve_machine(part_2)
                .map(|(a_num, b_num)| 3 * a_num + b_num)
        }

        ///Returns only a valid, checked solution to the machine if there is one or None
        fn solve_machine(&self, part_2: bool) -> Option<(u64, u64)> {
            self.check_solution(self.attempt_solve_machine(part_2), part_2)
        }

        ///Attempts to solve the machine,
        /// returning a potential solution that needs to be checked (due
        /// to potential rounding errors). The solution returned also needs to be checked
        /// for A_num, B_num <=100 for part_1.
        fn attempt_solve_machine(&self, part_2: bool) -> Option<(u64, u64)> {
            let solve = if !part_2 {
                Machine::solve_eq
            } else {
                Machine::solve_eq_part2
            };

            match *self {
                Machine {
                    a_x,
                    a_y,
                    b_x,
                    b_y,
                    p_x,
                    p_y,
                } if a_x == 0 && b_x == 0 => {
                    if p_x != 0 {
                        None
                    } else {
                        //So we have 1 equation with 2 variables
                        //A_y * A_num + B_y * B_num = Prize_y
                        solve(a_y, b_y, p_y)
                    }
                }
                Machine {
                    a_x,
                    a_y,
                    b_x,
                    b_y,
                    p_x,
                    p_y,
                } if a_y == 0 && b_y == 0 => {
                    if p_y != 0 {
                        None
                    } else {
                        //So we have 1 equation with 2 variables
                        //A_x * A_num + B_x * B_num = Prize_x
                        solve(a_x, b_x, p_x)
                    }
                }

                //We know *each* equation contains at least one non-zero coefficent in all following cases
                Machine {
                    a_x,
                    a_y,
                    b_x,
                    b_y,
                    p_x,
                    p_y,
                } if a_x != 0 => {
                    //solving
                    // A_x * A_num + B_x * B_num = Prize_x <--L1
                    // A_y * A_num + B_y * B_num = Prize_y <--L2
                    //L1 *(-A_y/A_x) +L2 --> L1
                    //(B_x *(-A_y/A_x) +B_y) * B_num = Prize_x *(-A_y/A_x) + Prize_Y
                    //solve for B_num and then use L1 to find A_num

                    let a_x: f64 = a_x as f64;
                    let a_y: f64 = a_y as f64;
                    let b_x: f64 = b_x as f64;
                    let b_y: f64 = b_y as f64;
                    let p_x: f64 = p_x as f64;
                    let p_y: f64 = p_y as f64;

                    if (b_x * (-a_y / a_x) + b_y) == 0.0 {
                        if (p_x * (-a_y / a_x) + p_y) == 0.0 {
                            //This means our equations are linearly-dependent!
                            //that is L1 *(-A_y/A_x) = -L2
                            //so we have one equation with two variables
                            solve(a_x as u64, b_x as u64, p_x as u64)
                        } else {
                            //In the Gaussian elimination matrix
                            //we have a row like [0, 0 | 1] so no solution
                            None
                        }
                    } else {
                        let b_num = (p_x * (-a_y / a_x) + p_y) / (b_x * (-a_y / a_x) + b_y);

                        let a_num = (p_x - (b_x * b_num)) / a_x;

                        if b_num < 0.0 || a_num < 0.0 {
                            None
                        } else {
                            //Note we make sure that a_num, b_num is positive and an integer
                            //before casting it to u64
                            Some((a_num.round() as u64, b_num.round() as u64))
                        }
                    }
                }

                Machine {
                    a_x,
                    a_y,
                    b_x,
                    b_y,
                    p_x,
                    p_y,
                } if b_x != 0 => {
                    //solving
                    // A_x * A_num + B_x * B_num = Prize_x <--L1
                    // A_y * A_num + B_y * B_num = Prize_y <--L2
                    //L1 *(-B_y/B_x) +L2 --> L1
                    //(A_x *(-B_y/B_x) +A_y) * A_num = Prize_x *(-B_y/B_x) + Prize_Y
                    //solve for A_num and then use L1 to find B_num

                    let a_x: f64 = a_x as f64;
                    let a_y: f64 = a_y as f64;
                    let b_x: f64 = b_x as f64;
                    let b_y: f64 = b_y as f64;
                    let p_x: f64 = p_x as f64;
                    let p_y: f64 = p_y as f64;

                    if (a_x * (-b_y / b_x) + a_y) == 0.0 {
                        if (p_x * (-b_y / b_x) + p_y) == 0.0 {
                            //This means our equations are linearly-dependent!
                            //that is L1 *(-A_y/A_x) = -L2
                            //so we have one equation with two variables
                            solve(a_x as u64, b_x as u64, p_x as u64)
                        } else {
                            //In the Gaussian elimination matrix
                            //we have a row like [0, 0 | 1] so no solution
                            None
                        }
                    } else {
                        let a_num = (p_x * (-b_y / b_x) + p_y) / (a_x * (-b_y / b_x) + a_y);

                        let b_num = (p_x - (a_x * a_num)) / b_x;

                        if b_num < 0.0 || a_num < 0.0 {
                            None
                        } else {
                            //Note we make sure that a_num, b_num is positive and an integer
                            //before casting it to u64
                            Some((a_num.round() as u64, b_num.round() as u64))
                        }
                    }
                }

                _ => unreachable!(),
            }
        }

        ///Solves an equation of the form A_i * A_num + B_i * B_num = Prize_i
        /// where i is x or y.
        fn solve_eq(a: u64, b: u64, prize: u64) -> Option<(u64, u64)> {
            if a == 0 && b == 0 {
                //We pick Some(0,0) because all solutions would work but this one minimizes the cost function
                //the most
                return match prize {
                    0 => Some((0, 0)),
                    _ => None,
                };
            } else if a == 0 {
                //we know from previous case b is not zero
                if prize % b == 0 {
                    return Some((0, prize / b));
                } else {
                    return None;
                }
            } else if b == 0 {
                //we know from previous case a is not zero
                if prize % a == 0 {
                    return Some((prize / a, 0));
                } else {
                    return None;
                }
            }

            //Greedy solve this by attempting to start with A_num =0 (pressing A is more expensive)
            // and going up (Note we know b !=0).
            //also note if a_num =0 then we do the same calculation as if a =0.
            for a_num in 1..=100 {
                if (a * a_num) > prize {
                    //This means b_num would be negative so it is not a solution
                    return None;
                }
                if (prize - (a * a_num)) % b == 0 {
                    return Some((a_num, (prize - (a * a_num)) / b));
                }
            }

            None
        }

        ///Returns Some(a_num, b_num)
        /// if a_num, b_num is a valid solution for this machine.
        /// That is they solve
        ///
        ///     A_x * A_num + B_x * B_num = Prize_x
        ///     A_y * A_num + B_y * B_num = Prize_y
        ///
        /// And we have A_num, B_num <=100 (for part 1 only!)
        fn check_solution(
            &self,
            potential_solution: Option<(u64, u64)>,
            part_2: bool,
        ) -> Option<(u64, u64)> {
            match potential_solution {
                Some((a_num, b_num)) => {
                    if (a_num > 100 || b_num > 100) && !part_2 {
                        None
                    } else if self.a_x * a_num + self.b_x * b_num == self.p_x
                        && self.a_y * a_num + self.b_y * b_num == self.p_y
                    {
                        Some((a_num, b_num))
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }

    //Part 2 specific stuff
    impl Machine {
        ///Solves an equation of the form A_i * A_num + B_i * B_num = Prize_i
        /// where i is x or y.
        fn solve_eq_part2(a: u64, b: u64, prize: u64) -> Option<(u64, u64)> {
            if a == 0 && b == 0 {
                //We pick Some(0,0) because all solutions would work but this one minimizes the cost function
                //the most
                return match prize {
                    0 => Some((0, 0)),
                    _ => None,
                };
            } else if a == 0 {
                //we know from previous case b is not zero
                if prize % b == 0 {
                    return Some((0, prize / b));
                } else {
                    return None;
                }
            } else if b == 0 {
                //we know from previous case a is not zero
                if prize % a == 0 {
                    return Some((prize / a, 0));
                } else {
                    return None;
                }
            }

            //Greedy solve this by attempting to start with A_num =0 (pressing A is more expensive)
            // and going up (Note we know b !=0).
            //also note if a_num =0 then we do the same calculation as if a =0.
            for a_num in 1.. {
                if (a * a_num) > prize {
                    //This means b_num would be negative so it is not a solution
                    return None;
                }
                if (prize - (a * a_num)) % b == 0 {
                    return Some((a_num, (prize - (a * a_num)) / b));
                }
            }

            None
        }
    }
}

///Returns the smallest number of tokens you would have to spend to win as many prizes as possible.
fn solution_part1(file_path: &str) -> u64 {
    //Note we have to make sure the End of Line Sequence is LF and NOT CRLF in day13example.txt

    let haystack: String = fs::read_to_string(file_path).expect("failed to open file");
    let haystack = haystack.trim();

    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(
        r"(?ms).?Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();

    println!("regex works: {}", re.is_match(haystack));

    re.captures_iter(haystack)
        .map(|c| {
            let (_, [a_x, a_y, b_x, b_y, p_x, p_y]) = c.extract();

            let (a_x, a_y, b_x, b_y, p_x, p_y) = (
                a_x.parse::<u64>().unwrap(),
                a_y.parse::<u64>().unwrap(),
                b_x.parse::<u64>().unwrap(),
                b_y.parse::<u64>().unwrap(),
                p_x.parse::<u64>().unwrap(),
                p_y.parse::<u64>().unwrap(),
            );

            let machine = Machine::new(a_x, a_y, b_x, b_y, p_x, p_y);
            //dbg!(&machine);

            machine.price(false).unwrap_or(0)
        })
        .sum()
}

///Returns the smallest number of tokens you would have to spend to win as many prizes as possible.
/// But we add 10000000000000 to each Prize X and Y coordinate first.
fn solution_part2(file_path: &str) -> u64 {
    const ADD: u64 = 10000000000000;

    //Note we have to make sure the End of Line Sequence is LF and NOT CRLF in day13example.txt

    let haystack: String = fs::read_to_string(file_path).expect("failed to open file");
    let haystack = haystack.trim();

    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(
        r"(?ms).?Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)",
    )
    .unwrap();

    println!("regex works: {}", re.is_match(haystack));

    re.captures_iter(haystack)
        .map(|c| {
            let (_, [a_x, a_y, b_x, b_y, p_x, p_y]) = c.extract();

            let (a_x, a_y, b_x, b_y, p_x, p_y) = (
                a_x.parse::<u64>().unwrap(),
                a_y.parse::<u64>().unwrap(),
                b_x.parse::<u64>().unwrap(),
                b_y.parse::<u64>().unwrap(),
                p_x.parse::<u64>().unwrap(),
                p_y.parse::<u64>().unwrap(),
            );

            let (p_x, p_y) = (p_x + ADD, p_y + ADD);

            let machine = Machine::new(a_x, a_y, b_x, b_y, p_x, p_y);
            //dbg!(&machine);

            machine.price(true).unwrap_or(0)
        })
        .sum()
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day13.txt"));
        dbg!(solution_part2("puzzle_inputs/day13.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day13example.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn example_part2() {
        assert!(10000000000000 <= u64::MAX);
        //We are not given the answer for part_2 for the example
    }
}
