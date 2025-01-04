use std::fs;

//Part 1 solution notes:
//Note the score for each trailhead can not exceed the number of 9's on the map.
//Breadth-First search is an ideal algorithm to solve this since we only care about reaching 9's
//and we know we reach them all in the exact same time.
//So do BFS and keep track of unique 9's positions on the map we reach.
//We could also do recursion -- but that means we might redo some work exploring the same nodes repeatdly
//for example if the map was
//01
//12
//Then we explore the 2 node twice unless we do BFS.

pub mod topographic_map {

    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    //We use Matrix as helper struct to TopMap
    ///A Matrix holds a vector of vectors of usize
    struct Matrix {
        data: Vec<Vec<usize>>,
        row_num: usize,
        col_num: usize,
    }

    impl Matrix {
        //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
        fn new(data: Vec<Vec<usize>>) -> Result<Matrix, &'static str> {
            if data.is_empty() {
                return Err("empty input");
            }

            let row_num = data.len();
            let col_num = data[0].len();

            Ok(Matrix {
                data,
                row_num,
                col_num,
            })
        }
    }

    pub struct TopMap {
        trail_head_locations: HashSet<(usize, usize)>,
        map: Matrix,
    }

    impl TopMap {
        pub fn build(map: Vec<Vec<usize>>) -> TopMap {
            let map = Matrix::new(map).unwrap();
            let trail_head_locations = TopMap::find_trailheads(&map).unwrap();

            TopMap {
                trail_head_locations,
                map,
            }
        }

        ///Find the locations of the trailheads (0's) in a map.
        ///That is find indices (i,j) such that self.map at (i,j) == 0
        fn find_trailheads(map: &Matrix) -> Result<HashSet<(usize, usize)>, &'static str> {
            let mut locations: HashSet<(usize, usize)> = HashSet::new();

            for (row_index, row) in map.data.iter().enumerate() {
                for (col_index, entry) in row.iter().enumerate() {
                    if *entry == 0 {
                        locations.insert((row_index, col_index));
                    }
                }
            }

            if locations.is_empty() {
                Err("Could not find trailheads")
            } else {
                Ok(locations)
            }
        }

        ///Returns the sum of the scores of all trailheads on a topographic map.
        pub fn score_map(&self) -> usize {
            //We score each individual trailhead and sum their scores
            self.trail_head_locations
                .iter()
                .map(|trailhead_location| self.score_trailhead(*trailhead_location))
                .sum()
        }

        ///Returns the number of 9's reachable from this location according to the hiking rules.
        fn score_trailhead(&self, head_location: (usize, usize)) -> usize {
            //We do Breadth-first search. See reasons why at the top of this file
            let mut queue = VecDeque::new();

            //The locations of the 9's reachable from this head location
            let mut end_of_trail_locations = HashSet::new();

            let mut explored = HashSet::new();

            queue.push_back(head_location);

            explored.insert(head_location);

            while let Some(location) = queue.pop_front() {
                //see if one space up, down, left, or right from the location has a value greater by exactly one
                //from the current location, if so enqueue it.
                //If that value happens to be a 9 then just insert it to end_of_trail_locations instead.
                let next_value = self.map.data[location.0][location.1] + 1;

                let trail_end = next_value == 9;

                //Look up. Note this is safe as && short-circuits in Rust
                //(the RHS is evaluated only if  location.0  > 0 is true).
                //Using explored allows us to avoid visiting nodes repeatdly.
                if location.0 > 0
                    && self.map.data[location.0 - 1][location.1] == next_value
                    && explored.insert((location.0 - 1, location.1))
                {
                    if !trail_end {
                        queue.push_back((location.0 - 1, location.1));
                    } else {
                        end_of_trail_locations.insert((location.0 - 1, location.1));
                    }
                }

                //Look down
                if location.0 + 1 < self.map.row_num
                    && self.map.data[location.0 + 1][location.1] == next_value
                    && explored.insert((location.0 + 1, location.1))
                {
                    if !trail_end {
                        queue.push_back((location.0 + 1, location.1));
                    } else {
                        end_of_trail_locations.insert((location.0 + 1, location.1));
                    }
                }

                //Look Right
                if location.1 + 1 < self.map.col_num
                    && self.map.data[location.0][location.1 + 1] == next_value
                    && explored.insert((location.0, location.1 + 1))
                {
                    if !trail_end {
                        queue.push_back((location.0, location.1 + 1));
                    } else {
                        end_of_trail_locations.insert((location.0, location.1 + 1));
                    }
                }

                //Look Left
                if location.1 > 0
                    && self.map.data[location.0][location.1 - 1] == next_value
                    && explored.insert((location.0, location.1 - 1))
                {
                    if !trail_end {
                        queue.push_back((location.0, location.1 - 1));
                    } else {
                        end_of_trail_locations.insert((location.0, location.1 - 1));
                    }
                }
            }

            end_of_trail_locations.len()
        }

        ///Returns the sum of all trailhead ratings in the map.
        /// A trailhead's rating is the number of *distinct* hiking trails that start with that trailhead.
        pub fn rate_map(&self) -> usize {
            //We rate each individual trailhead and sum their scores
            self.trail_head_locations
                .iter()
                .map(|trailhead_location| self.rate_trailhead(*trailhead_location))
                .sum()
        }

        ///Returns the number of distinct hiking trails that start with at head_location
        fn rate_trailhead(&self, head_location: (usize, usize)) -> usize {
            //Note: As the hiking trail must increase by a value of 1 at each step,
            //We can still do BFS but this time we use a HashMap instead of a HashSet for explored.
            //The value is the multipler when we reach a 9.
            //This way we still avoid visting the same node multiple times.

            //If hiking trails didn't work like this it would be problematic
            //for example
            //02
            //12
            //Then we could go down left up and by the time we reach the top-right 2 it would have already gone on
            //with an incorrect multiplier to other nodes.

            let mut queue = VecDeque::new();

            let mut explored: HashMap<(usize, usize), usize> = HashMap::new();

            queue.push_back(head_location);

            explored.insert(head_location, 1);

            while let Some(location) = queue.pop_front() {
                //see if one space up, down, left, or right from the location has a value greater by exactly one
                //from the current location, if so enqueue it.
                //If that value happens to be a 9 then just insert it to end_of_trail_locations instead.
                let next_value = self.map.data[location.0][location.1] + 1;
                let current_multiplier = *explored
                    .get(&location)
                    .expect("multipler should be initialized for locations queued");

                if next_value == 10 {
                    //We have reached a stage in the queue where the queue consists only of 9s
                    //Note we have to re-add this 9 back to the queue first.
                    queue.push_back(location);
                    break;
                }

                //Look up. Note this is safe as && short-circuits in Rust
                //(the RHS is evaluated only if  location.0  > 0 is true).
                //Using explored allows us to avoid visiting nodes repeatdly.
                if location.0 > 0 && self.map.data[location.0 - 1][location.1] == next_value {
                    //Update the multiplier
                    explored
                        .entry((location.0 - 1, location.1))
                        .and_modify(|e| *e += current_multiplier)
                        .or_insert(current_multiplier);

                    //add the node to the queue if it was not there already
                    if current_multiplier == *explored.get(&(location.0 - 1, location.1)).unwrap() {
                        queue.push_back((location.0 - 1, location.1));
                    }
                }

                //Look down
                if location.0 + 1 < self.map.row_num
                    && self.map.data[location.0 + 1][location.1] == next_value
                {
                    explored
                        .entry((location.0 + 1, location.1))
                        .and_modify(|e| *e += current_multiplier)
                        .or_insert(current_multiplier);

                    //add the node to the queue if it was not there already
                    if current_multiplier == *explored.get(&(location.0 + 1, location.1)).unwrap() {
                        queue.push_back((location.0 + 1, location.1));
                    }
                }

                //Look Right
                if location.1 + 1 < self.map.col_num
                    && self.map.data[location.0][location.1 + 1] == next_value
                {
                    explored
                        .entry((location.0, location.1 + 1))
                        .and_modify(|e| *e += current_multiplier)
                        .or_insert(current_multiplier);

                    //add the node to the queue if it was not there already
                    if current_multiplier == *explored.get(&(location.0, location.1 + 1)).unwrap() {
                        queue.push_back((location.0, location.1 + 1));
                    }
                }

                //Look Left
                if location.1 > 0 && self.map.data[location.0][location.1 - 1] == next_value {
                    explored
                        .entry((location.0, location.1 - 1))
                        .and_modify(|e| *e += current_multiplier)
                        .or_insert(current_multiplier);

                    //add the node to the queue if it was not there already
                    if current_multiplier == *explored.get(&(location.0, location.1 - 1)).unwrap() {
                        queue.push_back((location.0, location.1 - 1));
                    }
                }
            }

            //calcuate the rating of the trailhead. Our queue now consists of all reachable 9's
            //and we know how many times we reached each 9 due to the multiplier
            queue
                .into_iter()
                .map(|location| {
                    *explored
                        .get(&location)
                        .expect("multipler should be initialized for locations queued")
                })
                .sum()
        }
    }
}

fn solution_part1(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    char.to_digit(10)
                        .expect("Map should be composed of digits 0-9") as usize
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    let map = topographic_map::TopMap::build(data);
    map.score_map()
}

fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    char.to_digit(10)
                        .expect("Map should be composed of digits 0-9") as usize
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    let map = topographic_map::TopMap::build(data);
    map.rate_map()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day10.txt"));
        dbg!(solution_part2("puzzle_inputs/day10.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day10example.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day10example.txt");
        assert_eq!(result, 81);
    }
}
