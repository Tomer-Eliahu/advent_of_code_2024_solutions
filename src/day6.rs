use std::fs;

mod guard {
    use std::collections::HashSet;

    //We use Matrix as helper struct to GuardPatrol
    ///A Matrix holds a vector of vectors of chars
    struct Matrix {
        data: Vec<Vec<char>>,
        row_num: usize,
        col_num: usize,
    }

    impl Matrix {
        //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
        fn new(data: Vec<Vec<char>>) -> Matrix {
            if data.is_empty() {
                panic!("empty input")
            }

            let row_num = data.len();
            let col_num = data[0].len();

            Matrix {
                data,
                row_num,
                col_num,
            }
        }

        ///Find the location of the guard in a map.
        ///That is find indices (i,j) such that self.data at (i,j) =='^'
        fn find_guard(&self) -> Result<(usize, usize), &'static str> {
            for (row_index, row) in self.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    if *char == '^' {
                        return Ok((row_index, col_index));
                    }
                }
            }

            Err("Could not find guard")
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    enum GuardDirection {
        Up,
        Down,
        Left,
        Right,
    }

    pub struct GuardPatrol {
        guard_location: (usize, usize),
        guard_direction: GuardDirection,
        map: Matrix,
    }

    impl GuardPatrol {
        pub fn new(map: Vec<Vec<char>>) -> GuardPatrol {
            let map = Matrix::new(map);

            //The guard is initially facing up
            let guard_direction = GuardDirection::Up;

            let guard_location = map.find_guard().unwrap();

            GuardPatrol {
                guard_location,
                guard_direction,
                map,
            }
        }
        ///Advance the guard by 1 step (or turn) if possible,
        ///replacing the current position in the map with an 'X'.
        ///Returns Ok(()) if the guard is still in bounds of the map and Err("out_of_bounds") otherwise.
        fn traverse_with_trace(&mut self) -> Result<(), &'static str> {
            //first replace the current positon of the guard in the map with an X
            self.map.data[self.guard_location.0][self.guard_location.1] = 'X';

            //Move the guard 1 step (or turn) according to the direction the guard faces
            match self.guard_direction {
                GuardDirection::Up => {
                    //Check if 1 step up is not out of the bounds of the map
                    if self.guard_location.0 > 0 {
                        //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                        if self.map.data[self.guard_location.0 - 1][self.guard_location.1] == '#' {
                            self.guard_direction = GuardDirection::Right;
                        } else {
                            //move the guard 1 step up
                            self.guard_location =
                                (self.guard_location.0 - 1, self.guard_location.1);
                        }
                    } else {
                        //The guard goes out of bounds
                        return Err("out_of_bounds");
                    }
                }
                GuardDirection::Down => {
                    //Check if 1 step down is not out of the bounds of the map
                    if self.guard_location.0 + 1 < self.map.row_num {
                        //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                        if self.map.data[self.guard_location.0 + 1][self.guard_location.1] == '#' {
                            self.guard_direction = GuardDirection::Left;
                        } else {
                            //move the guard 1 step down
                            self.guard_location =
                                (self.guard_location.0 + 1, self.guard_location.1);
                        }
                    } else {
                        //The guard goes out of bounds
                        return Err("out_of_bounds");
                    }
                }
                GuardDirection::Right => {
                    //Check if 1 step right is not out of the bounds of the map
                    if self.guard_location.1 + 1 < self.map.col_num {
                        //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                        if self.map.data[self.guard_location.0][self.guard_location.1 + 1] == '#' {
                            self.guard_direction = GuardDirection::Down;
                        } else {
                            //move the guard 1 step right
                            self.guard_location =
                                (self.guard_location.0, self.guard_location.1 + 1);
                        }
                    } else {
                        //The guard goes out of bounds
                        return Err("out_of_bounds");
                    }
                }

                GuardDirection::Left => {
                    //Check if 1 step left is not out of the bounds of the map
                    if self.guard_location.1 > 0 {
                        //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                        if self.map.data[self.guard_location.0][self.guard_location.1 - 1] == '#' {
                            self.guard_direction = GuardDirection::Up;
                        } else {
                            //move the guard 1 step left
                            self.guard_location =
                                (self.guard_location.0, self.guard_location.1 - 1);
                        }
                    } else {
                        //The guard goes out of bounds
                        return Err("out_of_bounds");
                    }
                }
            }

            Ok(())
        }

        ///Make the guard do a full patrol, leaving a trace.
        ///Returns the number of distinct locations (including the initial guard location) the guard has been
        pub fn full_patrol(&mut self) -> usize {
            //traverse untill the error (guard goes out of bounds)
            //we are guaranteed the guard will go out of bounds at some point
            while self.traverse_with_trace().is_ok() {}

            //count (and return) the number of X's in the map after the patrol ended (guard went out of bounds)
            self.map
                .data
                .iter()
                .flatten()
                .filter(|&&char| char == 'X')
                .count()
        }

        ///find out if a guard patrol starting at self.guard_location with inital direction
        ///self.guard_direction loops foever or ends.
        ///Return 1 if it loops forever and 0 otherwise.
        fn obstructed_patrol(&self) -> usize {
            let mut current_location = self.guard_location;
            let mut current_direction = self.guard_direction;
            let mut travel_log = HashSet::new();

            //This terminates as the guard either patrols forever or they don't, meaning the guard goes out of
            //the bounds of the map at some point
            loop {
                //We create a new set (travel_log) and each step we move we record
                //in the travel log the direction we went (for example if at (i,j) we moved up
                //then add (i,j, GuardDirection::Up) to set of movements we made).
                //
                //If we ever go through the same place in the same direction we did before, then we know
                //the guard is on a loop and thus is going to be patroling forever

                //Note insert returns whether the value was newly inserted.
                if !travel_log.insert((current_location, current_direction)) {
                    //The guard is on a loop
                    return 1;
                }

                //Move the guard 1 step (or turn) according to the direction the guard faces
                match current_direction {
                    GuardDirection::Up => {
                        //Check if 1 step up is not out of the bounds of the map
                        if current_location.0 > 0 {
                            //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                            if self.map.data[current_location.0 - 1][current_location.1] == '#' {
                                current_direction = GuardDirection::Right;
                            } else {
                                //move the guard 1 step up
                                current_location = (current_location.0 - 1, current_location.1);
                            }
                        } else {
                            //The guard goes out of bounds
                            return 0;
                        }
                    }
                    GuardDirection::Down => {
                        //Check if 1 step down is not out of the bounds of the map
                        if current_location.0 + 1 < self.map.row_num {
                            //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                            if self.map.data[current_location.0 + 1][current_location.1] == '#' {
                                current_direction = GuardDirection::Left;
                            } else {
                                //move the guard 1 step down
                                current_location = (current_location.0 + 1, current_location.1);
                            }
                        } else {
                            //The guard goes out of bounds
                            return 0;
                        }
                    }
                    GuardDirection::Right => {
                        //Check if 1 step right is not out of the bounds of the map
                        if current_location.1 + 1 < self.map.col_num {
                            //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                            if self.map.data[current_location.0][current_location.1 + 1] == '#' {
                                current_direction = GuardDirection::Down;
                            } else {
                                //move the guard 1 step right
                                current_location = (current_location.0, current_location.1 + 1);
                            }
                        } else {
                            //The guard goes out of bounds
                            return 0;
                        }
                    }

                    GuardDirection::Left => {
                        //Check if 1 step left is not out of the bounds of the map
                        if current_location.1 > 0 {
                            //if there is an obstacle in front of the guard the guard turns 90 degrees right instead
                            if self.map.data[current_location.0][current_location.1 - 1] == '#' {
                                current_direction = GuardDirection::Up;
                            } else {
                                //move the guard 1 step left
                                current_location = (current_location.0, current_location.1 - 1);
                            }
                        } else {
                            //The guard goes out of bounds
                            return 0;
                        }
                    }
                }
            }
        }

        ///Get the number of distinct locations one can place an obstruction that
        ///will cause the guard to patrol forever
        pub fn count_unique_trap_locations(&self) -> usize {
            //We try placing the obstruction at each possible free location (not '^' or '#')
            //and observe if the guard then patrols forever
            let mut counter = 0;

            for char in self.map.data.iter().flatten() {
                //note if char is not '^' or '#' then it is '.'
                if *char == '.' {
                    //Logically this method is immutable since the mutations we do on the map
                    //are temporary
                    let raw_mut = char as *const char as *mut char;

                    //place a temporary obstruction at this char's location
                    unsafe {
                        raw_mut.write('#');
                    }

                    counter += self.obstructed_patrol();

                    //remove temporary obstruction at this char's location
                    unsafe {
                        raw_mut.write('.');
                    }
                }
            }

            counter
        }
    }
}

///Reads the input text and returns the number of distinct locations the guard will be on their patrol
fn solution_part1(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut patrol = guard::GuardPatrol::new(data);

    patrol.full_patrol()
}

///Reads the input text and returns the number of distinct locations one can place an obstruction that
/// will cause the guard to patrol forever
fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let patrol = guard::GuardPatrol::new(data);

    patrol.count_unique_trap_locations()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day6.txt"));
        dbg!(solution_part2("puzzle_inputs/day6.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day6example.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day6example.txt");
        assert_eq!(result, 6);
    }
}
