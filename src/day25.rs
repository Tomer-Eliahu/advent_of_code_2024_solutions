use std::fs;

use lock_and_key::LocksAndKeys;

//Part 1 solution notes: Need to use 2 HashMaps (as we can have duplicate keys and locks potentially).
//The key (for each HashMap) is the height numbers for the 5 columns (see the puzzle prompt)
// and the value is how many times we encountered that value.

//Then we can simply iterate over the locks and get the count of how many unique lock-key
//combinations that fit such a lock we have by simply summing up count_lock*count_key over all the keys that fit.

pub mod lock_and_key {
    use std::collections::HashMap;

    use regex::Regex;

    pub struct LocksAndKeys {
        locks: HashMap<[usize; 5], usize>,
        keys: HashMap<[usize; 5], usize>,
    }

    impl LocksAndKeys {
        ///Constructs the system of locks and keys.
        ///
        /// **Note:** Make sure data originates from a file where the End of Line Sequence is LF.
        pub fn build(data: String) -> LocksAndKeys {
            //We use a Regex, the following website was helpful https://rustexp.lpil.uk/

            //Remember the locks are schematics that have the top row filled (#) and the bottom row empty (.);
            //the keys have the top row empty and the bottom row filled.

            let re = Regex::new(r"(?m)(?s)((#){5}|(\.){5}).(((\.|#){5}.){5})(#{5}|\.{5})").unwrap();

            re.captures_iter(&data).fold(
                LocksAndKeys {
                    locks: HashMap::new(),
                    keys: HashMap::new(),
                },
                |mut locks_and_keys: LocksAndKeys, c| {
                    //kind is the type of value of the first row (either '#' or '.').
                    //desc is a string description of the lock/key (the inner 5 rows each of length 5).
                    //Note desc does not include the top or bottom row as those are just used to determine
                    //if this is a key or a lock and we just use kind for that.
                    let (_, [_, kind, desc, ..]) = c.extract::<6>();

                    //We count how many '#' we have in each of the 5 columns of desc (we count line by line).
                    let heights = desc.lines().fold([0; 5], |mut acc: [usize; 5], line| {
                        line.chars()
                            .map(|char| -> usize {
                                match char {
                                    '#' => 1,
                                    '.' => 0,
                                    _ => unreachable!(),
                                }
                            })
                            .enumerate()
                            .for_each(|(index, value)| acc[index] += value);

                        acc
                    });

                    match kind {
                        "#" => {
                            //This is a lock
                            locks_and_keys
                                .locks
                                .entry(heights)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                        "." => {
                            //This is a key
                            locks_and_keys
                                .keys
                                .entry(heights)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                        _ => unreachable!(),
                    }

                    locks_and_keys
                },
            )
        }

        ///Returns how many unique lock/key pairs fit together without overlapping in any column
        pub fn find_unique_pairs(&self) -> usize {
            self.locks
                .iter()
                .fold(0, |mut unique_pairs, (lock_heights, lock_count)| {
                    //Find out what is the max value for each column for the key_height
                    let mut max_key_heights = [5; 5];
                    lock_heights
                        .iter()
                        .enumerate()
                        .for_each(|(index, value)| max_key_heights[index] -= value);

                    for (key_heights, key_count) in &self.keys {
                        //We need to check for each index i from 0 to 4 that key_heights[i] <= max_key_heights[i]
                        let key_fits = (0..5).all(|i| key_heights[i] <= max_key_heights[i]);

                        if key_fits {
                            //We found a pair of lock and key values that fit.
                            unique_pairs += lock_count * key_count;
                        }
                    }

                    unique_pairs
                })
        }
    }
}

///Returns how many unique lock/key pairs fit together without overlapping in any column
fn solution_part1(file_path: &str) -> usize {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let door = LocksAndKeys::build(data);

    door.find_unique_pairs()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day25.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day25example.txt");
        assert_eq!(result, 3);
    }
}
