use std::fs;

use regex::Regex;

///Reads the input text and returns the sum of all mul(X,Y) operations
fn solution_part1() -> i32 {
    let raw_input = fs::read_to_string("puzzle_inputs/day3.txt").expect("failed to open file");

    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&raw_input)
        .map(|c| {
            let (_, [raw_x, raw_y]) = c.extract();
            let (x, y) = (raw_x.parse::<i32>().unwrap(), raw_y.parse::<i32>().unwrap());
            x * y
        })
        .sum()
}

///Reads the input text and returns the sum of all mul(X,Y) operations with do() and don't operations enabled
fn solution_part2() -> i32 {
    let raw_input = fs::read_to_string("puzzle_inputs/day3.txt").expect("failed to open file");

    let mut sum = 0;
    let mut mul_enabled = true;
    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    for capture in re.captures_iter(&raw_input) {
        //Note capture.get(0) is guaranteed to be Some(_)
        let instruction = capture.get(0).unwrap().as_str();

        if instruction == "do()" {
            mul_enabled = true;
        } else if instruction == "don't()" {
            mul_enabled = false;
        } else if mul_enabled
        //Note this else occurs only when the insturction is mul(X,Y)
        {
            let (raw_x, raw_y) = (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            );
            let (x, y) = (raw_x.parse::<i32>().unwrap(), raw_y.parse::<i32>().unwrap());
            sum += x * y;
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1());
        dbg!(solution_part2());
    }

    #[test]
    fn example_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let result: i32 = re
            .captures_iter(input)
            .map(|c| {
                let (_, [raw_x, raw_y]) = c.extract();
                let (x, y) = (raw_x.parse::<i32>().unwrap(), raw_y.parse::<i32>().unwrap());
                x * y
            })
            .sum();

        assert_eq!(result, 161);
    }

    #[test]
    fn example_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut sum = 0;
        let mut mul_enabled = true;
        //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        for capture in re.captures_iter(input) {
            //Note capture.get(0) is guaranteed to be Some(_)
            let instruction = capture.get(0).unwrap().as_str();

            if instruction == "do()" {
                mul_enabled = true;
            } else if instruction == "don't()" {
                mul_enabled = false;
            } else if mul_enabled
            //Note this else occurs only when the insturction is mul(X,Y)
            {
                let (raw_x, raw_y) = (
                    capture.get(1).unwrap().as_str(),
                    capture.get(2).unwrap().as_str(),
                );
                let (x, y) = (raw_x.parse::<i32>().unwrap(), raw_y.parse::<i32>().unwrap());
                sum += x * y;
            }
        }

        assert_eq!(sum, 48);
    }
}
