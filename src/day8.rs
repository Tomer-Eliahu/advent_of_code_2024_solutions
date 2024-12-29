use std::collections::HashMap;
use std::fs;

//part 1 solution notes:

//Simple but inefficent: do a pass on the input and create a hash map of signal: (locations of this signal antennas).
//Then iterate over the data matrix, for each location on the input map (the data matrix) we need to detremine if it has an antinode.
//For each location and each signal type (if that signal contains 2 or more antennas), calculate all distances from that location in a single pass
//(over enumerating each pair which means a lot of recalculating the same distances).
//Then for each distance find out if a distance twice its size has appeared (I think store the distances as a set).
//include short-circuting as all we want to know is that in a location are there distances X and 2X.

//Better: do a pass on the input and create a hash map of signal: (locations of this signal antennas).
//Then create a blank map of 0's (integers not a character).
//Then enumerate for each signal type, all possible pairs of that signal antennas, and change the corresponding blank map
//location from a 0 to a 1 whenever there is antinode in that location.
//Then the answer is merely the sum of the blank map (we use a blank map to avoid double counting).
//The trick is recongnizing that: Given a pair of same signal (say signal *) antennes A and B,
//the locations where the antinodes will be is simply how you travel from A to B repeated starting from B,
//and how you travel from B to A repeated starting from A. Look at the example on the website to see this.
//Actually, we can simply use a set instead of this blank map.

mod input {
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct InputMap {
        pub signal_locations: HashMap<char, Vec<(usize, usize)>>,
        pub row_num: usize,
        pub col_num: usize,
    }

    impl InputMap {
        pub fn count_unique_antinode_locations(&self) -> usize {
            //We use a set because we don't want to double count locations
            let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

            for (_signal, location_list) in self.signal_locations.iter() {
                if location_list.len() < 2 {
                    //1 antenna can't create an antinode
                    continue;
                }

                //given a Signal A, iterate over all combinations of 2 antennas of signal A
                for (&loc_1, &loc_2) in location_list.iter().tuple_combinations::<(_, _)>() {
                    self.add_antiondes(loc_1, loc_2, &mut antinode_locations);
                }
            }

            //So now our Hashset contains all the in-bounds locations of antinodes
            antinode_locations.len()
        }

        ///Given the locations of 2 antennas of signal A, add the antinodes that are in bounds to the HashSet of locations
        fn add_antiondes(
            &self,
            mut loc_1: (usize, usize),
            mut loc_2: (usize, usize),
            set: &mut HashSet<(usize, usize)>,
        ) {
            //a convinence for later (we make sure loc_1.0 <= loc_2.0)
            if loc_1.0 > loc_2.0 {
                (loc_1, loc_2) = (loc_2, loc_1);
            }

            let row_diff = loc_2.0 - loc_1.0;

            //so we want to add row_diff to loc_2 and subtract it from loc_1

            if loc_1.1 <= loc_2.1 {
                let col_diff: usize = loc_2.1 - loc_1.1;

                //so we want to add col_diff to loc_2 and subtract it from loc_1

                if row_diff <= loc_1.0 && col_diff <= loc_1.1 {
                    //we know it ok to do this subtraction (we stay in bounds)
                    set.insert((loc_1.0 - row_diff, loc_1.1 - col_diff));
                }

                let potential = (loc_2.0 + row_diff, loc_2.1 + col_diff);
                if potential.0 < self.row_num && potential.1 < self.col_num {
                    set.insert(potential);
                }
            } else {
                let col_diff: usize = loc_1.1 - loc_2.1;

                //so we want to subtract col_diff from loc_2 and add it to loc_1

                if row_diff <= loc_1.0 && loc_1.1 + col_diff < self.col_num {
                    set.insert((loc_1.0 - row_diff, loc_1.1 + col_diff));
                }

                if loc_2.0 + row_diff < self.row_num && col_diff <= loc_2.1 {
                    set.insert((loc_2.0 + row_diff, loc_2.1 - col_diff));
                }
            }
        }

        pub fn count_unique_antinode_locations_part2(&self) -> usize {
            //We use a set because we don't want to double count locations
            let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

            for (_signal, location_list) in self.signal_locations.iter() {
                if location_list.len() < 2 {
                    //1 antenna can't create an antinode
                    continue;
                }

                //given a Signal A, iterate over all combinations of 2 antennas of signal A
                for (&loc_1, &loc_2) in location_list.iter().tuple_combinations::<(_, _)>() {
                    self.add_antiondes_part2(loc_1, loc_2, &mut antinode_locations);
                }
            }

            //So now our Hashset contains all the in-bounds locations of antinodes
            antinode_locations.len()
        }

        ///Given the locations of 2 antennas of signal A, add the antinodes that are in bounds to the HashSet of locations.
        ///The trick to part 2 is to notice it is nearly identical to part 1 and the answer follows from
        ///pretending any new node we add to the HashSet of antinodes is its own antenna
        fn add_antiondes_part2(
            &self,
            mut loc_1: (usize, usize),
            mut loc_2: (usize, usize),
            set: &mut HashSet<(usize, usize)>,
        ) {
            //first off note that all antennas locations are also antinodes
            set.insert(loc_1);
            set.insert(loc_2);

            //a convinence for later (we make sure loc_1.0 <= loc_2.0)
            if loc_1.0 > loc_2.0 {
                (loc_1, loc_2) = (loc_2, loc_1);
            }

            let row_diff = loc_2.0 - loc_1.0;

            //so we want to add row_diff to loc_2 and subtract it from loc_1

            if loc_1.1 <= loc_2.1 {
                let col_diff: usize = loc_2.1 - loc_1.1;

                //so we want to add col_diff to loc_2 and subtract it from loc_1
                let mut current = loc_1;

                //We now add as many other antinodes in this line as possible while staying in bounds
                while row_diff <= current.0 && col_diff <= current.1 {
                    current = (current.0 - row_diff, current.1 - col_diff);
                    set.insert(current);
                }

                let mut current = loc_2;
                while current.0 + row_diff < self.row_num && current.1 + col_diff < self.col_num {
                    current = (current.0 + row_diff, current.1 + col_diff);
                    set.insert(current);
                }
            } else {
                let col_diff: usize = loc_1.1 - loc_2.1;

                //so we want to subtract col_diff from loc_2 and add it to loc_1

                let mut current = loc_1;

                while row_diff <= current.0 && current.1 + col_diff < self.col_num {
                    current = (current.0 - row_diff, current.1 + col_diff);
                    set.insert(current);
                }

                let mut current = loc_2;

                while current.0 + row_diff < self.row_num && col_diff <= current.1 {
                    current = (current.0 + row_diff, current.1 - col_diff);
                    set.insert(current);
                }
            }
        }
    }
}

///Returns how many unique locations within the bounds of the map contain an antinode
fn solution_part1(file_path: &str) -> usize {
    //A Hash Map of signal type as the key, and a vector of locations of that signal antennas as the value
    let mut signal_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut row_num = 0;
    let mut col_num = 0;

    for (line_num, line) in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .inspect(|_| {
            //count the number of rows
            row_num += 1;
        })
        .enumerate()
    {
        if line_num == 0 {
            for (col_num, char) in line
                .chars()
                .inspect(|_| {
                    //count the number of cols, the data is guaranteed to be rectanguler,
                    //so we only need to count this once
                    col_num += 1;
                })
                .enumerate()
            {
                if char != '.' {
                    //for each point on the input map, we update our hashmap accordingly
                    signal_locations
                        .entry(char)
                        .and_modify(|vec: &mut Vec<(usize, usize)>| vec.push((line_num, col_num)))
                        .or_insert(vec![(line_num, col_num)]);
                }
            }
        } else {
            for (col_num, char) in line.chars().enumerate() {
                if char != '.' {
                    //for each point on the input map, we update our hashmap accordingly
                    signal_locations
                        .entry(char)
                        .and_modify(|vec: &mut Vec<(usize, usize)>| vec.push((line_num, col_num)))
                        .or_insert(vec![(line_num, col_num)]);
                }
            }
        }
    }

    let input_map = input::InputMap {
        signal_locations,
        row_num,
        col_num,
    };

    input_map.count_unique_antinode_locations()
}

///Returns how many unique locations within the bounds of the map contain an antinode.
/// Note the distance doesn't matter now.
fn solution_part2(file_path: &str) -> usize {
    //A Hash Map of signal type as the key, and a vector of locations of that signal antennas as the value
    let mut signal_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut row_num = 0;
    let mut col_num = 0;

    for (line_num, line) in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .inspect(|_| {
            //count the number of rows
            row_num += 1;
        })
        .enumerate()
    {
        if line_num == 0 {
            for (col_num, char) in line
                .chars()
                .inspect(|_| {
                    //count the number of cols, the data is guaranteed to be rectanguler,
                    //so we only need to count this once
                    col_num += 1;
                })
                .enumerate()
            {
                if char != '.' {
                    //for each point on the input map, we update our hashmap accordingly
                    signal_locations
                        .entry(char)
                        .and_modify(|vec: &mut Vec<(usize, usize)>| vec.push((line_num, col_num)))
                        .or_insert(vec![(line_num, col_num)]);
                }
            }
        } else {
            for (col_num, char) in line.chars().enumerate() {
                if char != '.' {
                    //for each point on the input map, we update our hashmap accordingly
                    signal_locations
                        .entry(char)
                        .and_modify(|vec: &mut Vec<(usize, usize)>| vec.push((line_num, col_num)))
                        .or_insert(vec![(line_num, col_num)]);
                }
            }
        }
    }

    let input_map = input::InputMap {
        signal_locations,
        row_num,
        col_num,
    };

    input_map.count_unique_antinode_locations_part2()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day8.txt"));
        dbg!(solution_part2("puzzle_inputs/day8.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day8example.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day8example.txt");
        assert_eq!(result, 34);
    }
}
