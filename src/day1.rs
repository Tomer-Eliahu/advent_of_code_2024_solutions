use std::fs;
use std::collections::HashMap;


///Reads the input.txt file and returns a pair of vectors (left, right) where each vector is a list of location IDs
fn read_input() -> (Vec<i32>, Vec<i32>) {
    let (left, right): (Vec<_>, Vec<_>) = fs::read_to_string("puzzle_inputs/day1.txt")
        .expect("failed to open file")
        .lines()
        .map(|line| {
            let mut iter = line.splitn(2, ' ').map(|word| {
                word.trim() //Since there might be multiple spaces between each number, we need to trim
                    .parse::<i32>()
                    .expect("expected a number (parsing failed")
            });

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip();
    (left, right)
}

pub fn solution_part1() -> u32 {
    let (mut left, mut right) = read_input();
    left.sort_unstable();
    right.sort_unstable();
    let result: u32 = std::iter::zip(left, right)
        .map(|(x, y)| i32::abs_diff(x, y))
        .sum();
    dbg!(result);
    result
}

pub fn solution_part2() -> i32 {
    let (left, right) = read_input();
    
    //Note this problem becomes really easy if we convert the right list into a dictionary
    let mut right_dict = HashMap::new();
    for key in right {
        right_dict.entry(key).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut similarity_score= 0;
    for num in left {
        if let Some(&value) = right_dict.get(&num)
        {
            similarity_score = similarity_score + num * value;
        }
    }
    dbg!(similarity_score);
    similarity_score
   
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        solution_part1();
        solution_part2();
    }

    #[test]
    fn example_part1() {
        let input = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
        left.sort_unstable();
        right.sort_unstable();

        let result: u32 = std::iter::zip(left, right)
            .map(|(x, y)| i32::abs_diff(x, y))
            .sum();

        assert_eq!(result, 11); //11 is the answer
    }
}
