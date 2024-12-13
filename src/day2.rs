use std::fs;

enum LevelStatus {
    Inc,
    Dec,
    NotInitialized,
}

//We need to use generic type parameter here as only the complier knows the exact type of the Iterator
pub struct Report<T: Iterator<Item = i32>> {
    iter: T,
    level_status: LevelStatus,
}

impl<T> Report<T>
where
    T: Iterator<Item = i32>,
{
    pub fn new(iter: T) -> Report<T> {
        Report {
            iter,
            level_status: LevelStatus::NotInitialized,
        }
    }

    ///Evaluate a report returning 1 if it is safe and 0 otherwise
    pub fn evaluate(&mut self) -> usize {
        //We will iterate over the row and calculate the difference between each number and the previous number
        let mut previous_number = self.iter.next().expect("found an empty report (row)");

        for number in self.iter.by_ref() {
            //checks for integer overflow are on by default for debug and test profiles on Rust

            //We now proceed accourding to the level status
            match self.level_status {
                LevelStatus::NotInitialized => {
                    if 1 <= (number - previous_number) && (number - previous_number) <= 3 {
                        self.level_status = LevelStatus::Inc;
                        previous_number = number;
                    } else if -3 <= (number - previous_number) && (number - previous_number) <= -1 {
                        self.level_status = LevelStatus::Dec;
                        previous_number = number;
                    } else {
                        //This row is not a safe report
                        return 0;
                    }
                }
                LevelStatus::Inc => {
                    if 1 <= (number - previous_number) && (number - previous_number) <= 3 {
                        previous_number = number;
                    } else {
                        return 0;
                    }
                }
                LevelStatus::Dec => {
                    if -3 <= (number - previous_number) && (number - previous_number) <= -1 {
                        previous_number = number;
                    } else {
                        //This level is not a safe report
                        return 0;
                    }
                }
            }
        }

        //if we got here that means the report is safe so return 1
        1
    }

    ///Evaluate a report returning 1 if it is safe and 0 otherwise. The difference between this and Report::evalute
    /// is that this tolerates 1 bad level (1 bad number) in a row (in a report)
    pub fn evaluate_with_tolerance(mut self) -> usize {
        //When we encounter a bad number, we don't know if we should remove that number or the previous one
        //or maybe even a further back number.

        //10 12 10 8 6 -- In this case we must remove the first 10

        //11 14 8 7 6 -- In this case we must remove 14

        //10 11 80 12 13 14 -- in this case we must remove 80

        //Note that once we hit the fourth member in a row,
        //that removing any member before current_index -2 (that is index <= current_index -3)
        //cannot suddenly make this a safe report (think of each 2 numbers in a row as connected by an edge to see why).
        //so it is enough to look back at index -2 at most

        //We utlize vectors to avoid copying data
        let full_report: Vec<i32> = self.iter.collect();

        if full_report.is_empty() {
            panic!("empty report")
        };

        let mut index = 1;

        while index < full_report.len() {
            //We now proceed accourding to the level status
            match self.level_status {
                LevelStatus::NotInitialized => {
                    if 1 <= (full_report[index] - full_report[index - 1])
                        && (full_report[index] - full_report[index - 1]) <= 3
                    {
                        self.level_status = LevelStatus::Inc;
                        index += 1;
                    } else if -3 <= (full_report[index] - full_report[index - 1])
                        && (full_report[index] - full_report[index - 1]) <= -1
                    {
                        self.level_status = LevelStatus::Dec;
                        index += 1;
                    } else {
                        //We encountered a bad level

                        //We need to either remove this number or the previous number.
                        //In other words we need to figure out if to skip this index or the previous one
                        //(we know we must be at index 1)
                        if evaluate_row_as_vector(&full_report, Some(index)) == 1
                            || evaluate_row_as_vector(&full_report, Some(index - 1)) == 1
                        {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }

                //Note for the following cases index is at least 2
                LevelStatus::Inc => {
                    if 1 <= (full_report[index] - full_report[index - 1])
                        && (full_report[index] - full_report[index - 1]) <= 3
                    {
                        index += 1;
                    } else {
                        //We encountered a bad level

                        //We need to either remove this number or the previous number or 2 numbers ago.
                        if evaluate_row_as_vector(&full_report, Some(index)) == 1
                            || evaluate_row_as_vector(&full_report, Some(index - 1)) == 1
                            || evaluate_row_as_vector(&full_report, Some(index - 2)) == 1
                        {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }
                LevelStatus::Dec => {
                    if -3 <= (full_report[index] - full_report[index - 1])
                        && (full_report[index] - full_report[index - 1]) <= -1
                    {
                        index += 1;
                    } else {
                        //We encountered a bad level

                        //We need to either remove this number or the previous number or 2 numbers ago.
                        if evaluate_row_as_vector(&full_report, Some(index)) == 1
                            || evaluate_row_as_vector(&full_report, Some(index - 1)) == 1
                            || evaluate_row_as_vector(&full_report, Some(index - 2)) == 1
                        {
                            return 1;
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }

        //if we got here that means the report is safe so return 1
        1
    }
}

//Evaluates a report, skipping the index to skip (it it is not None), and returns whether the report is safe or not (no tolerance of bad level)
pub fn evaluate_row_as_vector(vec: &[i32], index_to_skip: Option<usize>) -> usize {
    if vec.len() <= 1 {
        return 1;
    }
    //From here on out we know vec.len() >= 2

    let mut status = LevelStatus::NotInitialized;
    let mut previous_number = &vec[0];
    let mut skip_index = -1;

    if let Some(num) = index_to_skip {
        if num == 0 {
            return evaluate_row_as_vector(&vec[1..], None);
        } else {
            skip_index = num as i32;
        }
    }

    for (index, number) in vec.iter().enumerate().skip(1) {
        if index as i32 == skip_index {
            continue;
        }

        //We now proceed accourding to the level status
        match status {
            LevelStatus::NotInitialized => {
                if 1 <= (number - previous_number) && (number - previous_number) <= 3 {
                    status = LevelStatus::Inc;
                    previous_number = number;
                } else if -3 <= (number - previous_number) && (number - previous_number) <= -1 {
                    status = LevelStatus::Dec;
                    previous_number = number;
                } else {
                    //This row is not a safe report
                    return 0;
                }
            }
            LevelStatus::Inc => {
                if 1 <= (number - previous_number) && (number - previous_number) <= 3 {
                    previous_number = number;
                } else {
                    //This row is not a safe report
                    return 0;
                }
            }
            LevelStatus::Dec => {
                if -3 <= (number - previous_number) && (number - previous_number) <= -1 {
                    previous_number = number;
                } else {
                    //This row is not a safe report
                    return 0;
                }
            }
        }
    }
    //if we got here that means this row is safe
    1
}

///Reads the input.txt file corrsponding to filepath (each row is a list of numbers separated by a space)
///and returns the number of safe reports
fn solution_part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| {
            //Take each line and map it to 1 if it corrsponds to a safe report and 0 otherwise

            //Convert a row into an iterator over i32 (note this is lazy)
            let iter = line.split(' ').map(|num| {
                num.parse::<i32>()
                    .expect("expected a number (parsing failed")
            });
            let mut report = Report::new(iter);
            report.evaluate()
        })
        .sum()
}

///Reads the input.txt file corrsponding to filepath (each row is a list of numbers separated by a space)
///and returns the number of safe reports with a tolerance for 1 bad level (number)
fn solution_part2(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| {
            //Take each line and map it to 1 if it corrsponds to a safe report and 0 otherwise

            //Convert a row into an iterator over i32 (note this is lazy)
            let iter = line.split(' ').map(|num| {
                num.parse::<i32>()
                    .expect("expected a number (parsing failed")
            });
            let report = Report::new(iter);
            report.evaluate_with_tolerance()
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day2.txt"));
        dbg!(solution_part2("puzzle_inputs/day2.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day2example.txt");
        assert_eq!(result, 2); //2 reports are safe
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day2example.txt");
        assert_eq!(result, 4); //4 reports are safe
    }
}
