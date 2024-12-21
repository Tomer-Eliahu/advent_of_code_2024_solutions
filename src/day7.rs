use std::fs;
pub mod equation {

    ///Returns whether the equation can evaluate to the goal if we select correct operations (operations available: + and *)
    pub fn is_true(goal: usize, current_value: usize, equation: &str) -> bool {
        //first we get the next number from the equation
        let (next_operand_raw, equation, equation_over) = match equation.split_once(" ") {
            Some((next_operand_raw, equation)) => (next_operand_raw, equation, false),
            None => (equation, "", true), //meaning we got to the end of the equation it now contains just the final number
        };

        let next_operand = next_operand_raw.parse::<usize>().unwrap();

        let (add_result, mul_result) = (current_value + next_operand, current_value * next_operand);

        if equation_over {
            //return whether adding or multiplying the final operand got us to the goal
            goal == add_result || goal == mul_result
        } else {
            //Short circuting: If both adding and multipying the current operand exceeds the goal than
            //this equation cannot be true.
            //Note we can't rely on just adding exceeding the goal because the current operand
            //can be 1 and num +1 > num *1. Note that we know all the number in the equation are positive whole numbers (no 0's).

            //we can optimize this more
            //i.e. if  goal < mul_result then no point in running is_true(goal, mul_result, equation).
            if goal < add_result && goal < mul_result {
                return false;
            }

            is_true(goal, add_result, equation) || is_true(goal, mul_result, equation)
        }
    }

    ///Returns whether the equation can evaluate to the goal if we select correct operations (operations available: +, *, ||).
    pub fn is_true_inc_concat(goal: usize, current_value: Option<usize>, equation: &str) -> bool {
        //first we get the next number from the equation
        let (next_operand_raw, equation, equation_over) = match equation.split_once(" ") {
            Some((next_operand_raw, equation)) => (next_operand_raw, equation, false),
            None => (equation, "", true), //meaning we got to the end of the equation it now contains just the final number
        };

        //We now can add, multiply or concat the next number to the current value
        let next_operand = next_operand_raw.parse::<usize>().unwrap();

        match current_value {
            Some(current_value) => {
                let concat_result = format!("{}{}", current_value, next_operand_raw)
                    .parse::<usize>()
                    .unwrap();
                let (add_result, mul_result) =
                    (current_value + next_operand, current_value * next_operand);

                if equation_over {
                    goal == add_result || goal == mul_result || goal == concat_result
                } else {
                    //Short circuting: If both adding and multipying the current operand exceeds the goal than
                    //this equation cannot be true.
                    //Note we can't rely on just adding exceeding the goal because the current operand
                    //can be 1 and num +1 > num *1. Note that we know all the number in the equation are positive whole numbers (no 0's).
                    //Note that concating num2 to num1 is effectively = num1 * (10**number of digits in num2) + num2.
                    //So in particular concat_result > add_result.
                    if goal < add_result && goal < mul_result {
                        return false;
                    }

                    is_true_inc_concat(goal, Some(add_result), equation)
                        || is_true_inc_concat(goal, Some(mul_result), equation)
                        || is_true_inc_concat(goal, Some(concat_result), equation)
                }
            }
            None => {
                //If current_value was None then the results of concat,add, or multiply are just next_operand itself
                if equation_over {
                    goal == next_operand
                } else {
                    if goal < next_operand {
                        return false;
                    }

                    is_true_inc_concat(goal, Some(next_operand), equation)
                }
            }
        }
    }
}
///Returns the sum of the results of all true equations
fn solution_part1(file_path: &str) -> usize {
    let mut sum = 0;

    for line in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
    {
        let (result, equation) = line.split_once(":").expect("equation should contain ':'");
        let result = result.parse::<usize>().unwrap();
        let equation = equation.trim(); //equations look like 190: 10 19 so a trim is needed.

        let (first_num, equation) = equation
            .split_once(" ")
            .expect("An equation should contain at least two numbers");

        //It is ok we intialize current value like this as we know equations contain at least 2 numbers
        let current_value = first_num.parse::<usize>().unwrap();

        if equation::is_true(result, current_value, equation) {
            sum += result;
        }
    }
    sum
}

///Returns the sum of the results of all true equations (now accounting for || as well)
fn solution_part2(file_path: &str) -> usize {
    let mut sum = 0;

    for line in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
    {
        let (result, equation) = line.split_once(":").expect("equation should contain ':'");
        let result = result.parse::<usize>().unwrap();
        let equation = equation.trim(); //equations look like 190: 10 19 so a trim is needed.

        if equation::is_true_inc_concat(result, None, equation) {
            sum += result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day7.txt"));
        dbg!(solution_part2("puzzle_inputs/day7.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day7example.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day7example.txt");
        assert_eq!(result, 11387);
    }
}
