use std::fs;

use regex::Regex;

//Part 1 solution notes: Each byte position is given as an X,Y coordinate,
//where X is the distance from the left edge of your memory space
//and Y is the distance from the top edge of your memory space.

//Instead of using a Vector of Vectors to simulate where bytes land, we can just keep track of this in
//a HashSet with keys (usize, usize). We then want to keep track of the maze-runner position.
//We can use BFS (Breadth-First Search) to get the resulting shortest path length.

pub mod computer {

    use std::{collections::HashSet, mem};

    pub struct Maze {
        bytes: Vec<(usize, usize)>,
        map: HashSet<(usize, usize)>,
        size: (usize, usize),
    }

    impl Maze {
        pub fn new(bytes: Vec<(usize, usize)>, size: (usize, usize)) -> Maze {
            Self {
                bytes,
                map: HashSet::new(),
                size,
            }
        }

        ///Takes as input how many bytes to simulate falling, impacting the maze map.
        pub fn bytes_fall(&mut self, num: usize) {
            self.map.extend(self.bytes.drain(..num));
        }

        ///Solves the maze (with the map being the current Self.map value).
        /// Returns the minimum number of steps to solve the maze
        /// (the shortest-path length from the start of the maze to the end)
        ///  or None if the maze is not solvable.
        pub fn solve_maze(&self) -> Option<usize> {
            //We solve the maze using BFS.
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            //The runner starts at (0,0)
            visited.insert((0, 0));

            let mut visit_next: Vec<(usize, usize)> = vec![(0, 0)];

            let mut path_len = 0;

            'outer: loop {
                let mut visit_now: Vec<(usize, usize)> = vec![];

                //make visit_now = visit_next and empty visit_next
                mem::swap(&mut visit_now, &mut visit_next);

                if visit_now.is_empty() {
                    //Meaning there is no path out of this maze
                    break 'outer None;
                }

                while let Some(loc) = visit_now.pop() {
                    //Check if we are at the end of the maze
                    if loc == (self.size.0 - 1, self.size.1 - 1) {
                        break 'outer Some(path_len);
                    }

                    //See if the runner can go up
                    //(meaning the runner stays within the map and there is no obstacle there)
                    //and that we have yet to enqueue that location
                    if loc.0 > 0
                        && !self.map.contains(&(loc.0 - 1, loc.1))
                        && visited.insert((loc.0 - 1, loc.1))
                    {
                        visit_next.push((loc.0 - 1, loc.1));
                    }

                    //See if the runner can go down
                    if loc.0 + 1 < self.size.0
                        && !self.map.contains(&(loc.0 + 1, loc.1))
                        && visited.insert((loc.0 + 1, loc.1))
                    {
                        visit_next.push((loc.0 + 1, loc.1));
                    }

                    //See if the runner can go left
                    if loc.1 > 0
                        && !self.map.contains(&(loc.0, loc.1 - 1))
                        && visited.insert((loc.0, loc.1 - 1))
                    {
                        visit_next.push((loc.0, loc.1 - 1));
                    }

                    //See if the runner can go right
                    if loc.1 + 1 < self.size.1
                        && !self.map.contains(&(loc.0, loc.1 + 1))
                        && visited.insert((loc.0, loc.1 + 1))
                    {
                        visit_next.push((loc.0, loc.1 + 1));
                    }
                }
                //Our path length grew by 1
                path_len += 1;
            }
        }

        ///Returns the coordinates of the first byte
        /// that will prevent the exit from being reachable from the starting position.
        ///
        /// The brute force approach of letting every byte fall and then seeing if the maze is solvable
        /// takes too long. We can instead use successive halving to find the bad byte.
        pub fn find_bad_byte(mut self) -> (usize, usize) {
            let mut search_start = 0;

            let mut search_end = self.bytes.len() - 1;

            loop {
                let mid = search_start + (search_end - search_start) / 2;
                if (search_end - search_start) / 2 == 0 {
                    if search_start == search_end {
                        //We found the bad byte
                        return self.bytes[search_start];
                    } else {
                        //meaning there is a difference of 1 between search start and end
                        let bad_byte_start = self.bytes[search_start];
                        let bad_byte_end = self.bytes[search_end];

                        //We know that everything up untill search_start (non-inclusive)
                        //has already been marked on the map at this point
                        self.map.insert(bad_byte_start);

                        if self.solve_maze().is_none() {
                            return bad_byte_start;
                        } else {
                            //this means search end must be the bad_byte.
                            //Since we know the bad_byte must exist
                            return bad_byte_end;
                        }
                    }
                }

                //Because we are taking ownership of Self, we can optimize this function.
                //instead of doing something like

                //let mut test_maze = Maze::new(self.bytes.clone(), self.size);
                //test_maze.bytes_fall(mid); //get rid of all bytes with index in [0, mid)

                //We can do the following
                let mark_on_map = &self.bytes[search_start..mid]; // mark [start, mid)
                self.map.extend(mark_on_map);

                match self.solve_maze() {
                    Some(_) => {
                        //This means the bad byte must be have index in [mid, end]
                        search_start = mid;
                    }
                    None => {
                        //This means the bad byte must be have index in [start, mid-1]

                        //So we want to unmark [star, mid-1] from the map
                        //(note this relies on the fact we know bytes has no duplicates)
                        for value in mark_on_map {
                            self.map.remove(value);
                        }

                        search_end = mid - 1;
                    }
                }
            }
        }
    }
}

/// Simulates the first kilobyte (1024 bytes) falling onto your memory space (71 by 71).
/// Returns the minimum number of steps needed to reach the exit (the bottom right i.e coordinate (70,70))
/// From the top left (i.e. (0,0)).
fn solution_part1(file_path: &str, mem_space_size: (usize, usize)) -> usize {
    let raw_input = fs::read_to_string(file_path).expect("failed to open file");

    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let data: Vec<(usize, usize)> = re
        .captures_iter(&raw_input)
        .map(|c| {
            let (_, [raw_x, raw_y]) = c.extract();
            let (x, y) = (
                raw_x.parse::<usize>().unwrap(),
                raw_y.parse::<usize>().unwrap(),
            );
            (x, y)
        })
        .collect();

    let mut maze = computer::Maze::new(data, mem_space_size);

    if file_path == "puzzle_inputs/day18example.txt" {
        maze.bytes_fall(12);
    } else {
        maze.bytes_fall(1024);
    }

    maze.solve_maze().expect("part 1 is solvable")
}

///Returns the coordinates of the first byte that will prevent the exit from being reachable from the starting position
pub fn solution_part2(file_path: &str, mem_space_size: (usize, usize)) -> (usize, usize) {
    let raw_input = fs::read_to_string(file_path).expect("failed to open file");

    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let data: Vec<(usize, usize)> = re
        .captures_iter(&raw_input)
        .map(|c| {
            let (_, [raw_x, raw_y]) = c.extract();
            let (x, y) = (
                raw_x.parse::<usize>().unwrap(),
                raw_y.parse::<usize>().unwrap(),
            );
            (x, y)
        })
        .collect();

    let maze = computer::Maze::new(data, mem_space_size);

    maze.find_bad_byte()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day18.txt", (71, 71)));
        dbg!(solution_part2("puzzle_inputs/day18.txt", (71, 71)));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day18example.txt", (7, 7));
        assert_eq!(result, 22);
    }
}
