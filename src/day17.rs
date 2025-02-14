use computer::Computer;

pub mod computer {

    use std::mem;

    use itertools::Itertools;

    pub struct Computer<const N: usize> {
        register_a: usize,
        register_b: usize,
        register_c: usize,

        program: [usize; N],
        instruction_pointer: usize,
        output: Vec<usize>,
    }

    //We do this to avoid type errors later
    struct Combo(usize);

    impl<const N: usize> Computer<N> {
        pub const fn new(
            register_a: usize,
            register_b: usize,
            register_c: usize,
            program: [usize; N],
        ) -> Self {
            Computer {
                register_a,
                register_b,
                register_c,
                program,
                instruction_pointer: 0,
                output: vec![],
            }
        }

        ///Returns the lowest positive initial value for register A that
        /// causes the program to output a copy of itself.
        /// Unfortunetly, the solution involves observing what our particular puzzle input program
        /// does per iteration (the brute force approach takes too long).
        ///
        ///
        ///    One iteration of our program does:
        ///    B <- A mod 8
        ///    B <- B XOR 3 which is B XOR 011
        ///    C <- A/2.pow(B) which is shift A to the right B times
        ///    B <- B XOR C
        ///    A <- A/8 this is shifting A to the right 3 times
        ///    B <- B XOR 5 which is B XOR 101
        ///    OUTPUT <- B mod 8
        ///    (go back to start of program or halt)
        ///
        ///
        /// In particular we note that the A_new is just A_old shifted right 3 times.
        /// We also note that 3 bits of A output a digit. This means A must be 48 bits (our program length is 16; 16*3 =48).
        /// Note that to determine the last output digit we only care about the last 3 bits of A.
        /// To determine the last 2 output digits we only care about the last 6 bits of A.
        ///
        /// That means we can fill A by increaments of 3 bits from left to right.
        pub fn find_copy(self) -> usize {
            //Note the last digit we output (i.e. 0) is at the top of the stack
            let mut output_stack: Vec<usize> = self.program.into_iter().collect();

            assert_eq!(0, output_stack.pop().unwrap());

            //We use solutions to record potential values of a_in_bits that work so far.
            //The first 3 bits of A must be some value so that the program outputs 0 and halts.
            //We filter out potential solutions starting from a list of all possibilites
            let solutions: Vec<Vec<usize>> = vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ]
            .into_iter()
            .filter(|sol| Self::iteration(sol, "") == 0)
            .collect();
            //solutions at this point turns out to be just [1,1,0] which is a suprise.

            let final_values = Self::progress(solutions, output_stack);

            final_values
                .into_iter()
                .map(|sol: Vec<usize>| {
                    let string_value = sol.iter().map(ToString::to_string).join("");
                    usize::from_str_radix(&string_value, 2)
                        .expect("should be able to convert binary to number")
                })
                .min()
                .expect("should be at least one solution")
        }

        ///Figure out the value of A 3 bits at a time.
        /// Returns a Vector of *potential* solutions of a value for A such that the program outputs itself.
        fn progress(
            mut next_solutions: Vec<Vec<usize>>,
            mut output_stack: Vec<usize>,
        ) -> Vec<Vec<usize>> {
            while let Some(next_digit) = output_stack.pop() {
                println!(
                    "Before attempting to find solution for {next_digit}, we have {} solutions",
                    next_solutions.len()
                );

                let mut current_solutions = vec![];
                mem::swap(&mut current_solutions, &mut next_solutions);
                //Now next solutions is empty and current_solutions is what old next_solutions was.

                while let Some(a_in_bits) = current_solutions.pop() {
                    //We now want to find 3 bits to append to a_in_bits such that
                    //Running current_A as the value for register A yields output
                    //whose first digit is next_digit.

                    //Each solution we find we push to next_solutions.

                    for potential in 0..=7 {
                        match potential {
                            0 => {
                                if Self::iteration(&a_in_bits, "000") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([0, 0, 0]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            1 => {
                                if Self::iteration(&a_in_bits, "001") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([0, 0, 1]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            2 => {
                                if Self::iteration(&a_in_bits, "010") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([0, 1, 0]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            3 => {
                                if Self::iteration(&a_in_bits, "011") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([0, 1, 1]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            4 => {
                                if Self::iteration(&a_in_bits, "100") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([1, 0, 0]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            5 => {
                                if Self::iteration(&a_in_bits, "101") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([1, 0, 1]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            6 => {
                                if Self::iteration(&a_in_bits, "110") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([1, 1, 0]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            7 => {
                                if Self::iteration(&a_in_bits, "111") == next_digit {
                                    //We found a potential solution
                                    let mut new_sol = a_in_bits.clone();
                                    new_sol.extend([1, 1, 1]);

                                    next_solutions.push(new_sol);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }

            next_solutions
        }

        ///Does 1 iteration of our program on a_in_bits, returning what the output digit is.
        fn iteration(a_in_bits: &[usize], bits_to_append: &str) -> usize {
            let mut a_string = a_in_bits.iter().map(ToString::to_string).join("");
            a_string.push_str(bits_to_append);
            let a = usize::from_str_radix(&a_string, 2)
                .expect("should be able to convert binary to number");
            let mut b = a % 8;
            b ^= 3;
            let c = a / (2_usize.pow(b.try_into().unwrap()));
            b ^= c;
            //a gets divided by 8 but that does not impact the output
            b ^= 5;
            b %= 8;
            b
        }

        ///fetch the next instruction (opcode and operand)
        fn fetch_next(&self) -> Result<(usize, usize), &'static str> {
            let opcode = self.program.get(self.instruction_pointer).ok_or("halted")?;
            let operand = self
                .program
                .get(self.instruction_pointer + 1)
                .ok_or("halted")?;

            Ok((*opcode, *operand))
        }

        ///Converts an operand from a literal operand to a combo operand
        fn convert_to_combo(&self, operand: usize) -> Combo {
            let value = match operand {
                0..=3 => operand,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                7 => panic!("Invalid program (7 should not be a combo operand)"),
                _ => unreachable!(),
            };

            Combo(value)
        }

        ///Runs the computer until it halts. Returns the final output of the computer
        pub fn run(&mut self) -> String {
            while self.execute_next().is_ok() {}

            self.output.iter().map(ToString::to_string).join(",")
        }

        ///Makes the computer execute the next instruction
        fn execute_next(&mut self) -> Result<(), &str> {
            let (opcode, operand) = self.fetch_next()?;

            match opcode {
                0 => self.adv(self.convert_to_combo(operand)),
                1 => self.bxl(operand),
                2 => self.bst(self.convert_to_combo(operand)),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(self.convert_to_combo(operand)),
                6 => self.bdv(self.convert_to_combo(operand)),
                7 => self.cdv(self.convert_to_combo(operand)),
                _ => unreachable!(),
            }

            Ok(())
        }

        //The following instructions are named and implemented according to the puzzle input

        fn adv(&mut self, combo: Combo) {
            let numerator = self.register_a;
            let denominator = 2_usize
                .checked_pow(combo.0.try_into().unwrap())
                .expect("no overflow should occur");
            self.register_a = numerator / denominator;

            self.instruction_pointer += 2;
        }

        fn bxl(&mut self, literal: usize) {
            //In Rust '^' does bitwise XOR.
            //Because this is XOR we don't have to worry about the types being usize as opposed to say u8.
            self.register_b ^= literal;

            self.instruction_pointer += 2;
        }

        fn bst(&mut self, combo: Combo) {
            //As combo.0 is guaranteed to be non-negative, using reminder as opposed to "mod" is fine.
            self.register_b = combo.0 % 8;

            self.instruction_pointer += 2;
        }

        fn jnz(&mut self, literal: usize) {
            if self.register_a != 0 {
                //jump
                self.instruction_pointer = literal;
            } else {
                self.instruction_pointer += 2;
            }
        }

        fn bxc(&mut self, _literal: usize) {
            self.register_b ^= self.register_c;

            self.instruction_pointer += 2;
        }

        fn out(&mut self, combo: Combo) {
            self.output.push(combo.0 % 8);

            self.instruction_pointer += 2;
        }

        fn bdv(&mut self, combo: Combo) {
            let numerator = self.register_a;
            let denominator = 2_usize
                .checked_pow(combo.0.try_into().unwrap())
                .expect("no overflow should occur");
            self.register_b = numerator / denominator;

            self.instruction_pointer += 2;
        }

        fn cdv(&mut self, combo: Combo) {
            let numerator = self.register_a;
            let denominator = 2_usize
                .checked_pow(combo.0.try_into().unwrap())
                .expect("no overflow should occur");
            self.register_c = numerator / denominator;

            self.instruction_pointer += 2;
        }
    }
}

//For this problem using regex seems kinda of ridiculous so I just hardcode the input.

fn solution_part1() {
    //We can hardcode the input entirely in complie-time.
    let mut comp = const {
        Computer::new(
            30118712,
            0,
            0,
            [2, 4, 1, 3, 7, 5, 4, 2, 0, 3, 1, 5, 5, 5, 3, 0],
        )
    };

    let output = comp.run();
    println!("The final output is:\n{}", output);
}

///Returns the lowest positive initial value for register A that causes the program to output a copy of itself
pub fn solution_part2() -> usize {
    let input = const {
        Computer::new(
            30118712,
            0,
            0,
            [2, 4, 1, 3, 7, 5, 4, 2, 0, 3, 1, 5, 5, 5, 3, 0],
        )
    };

    input.find_copy()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        solution_part1();
        dbg!(solution_part2());
    }

    #[test]
    fn example_part1() {
        let mut comp = const { Computer::new(729, 0, 0, [0, 1, 5, 4, 3, 0]) };
        let output = comp.run();

        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
        println!("The final output is:\n{}", output);
    }
}
