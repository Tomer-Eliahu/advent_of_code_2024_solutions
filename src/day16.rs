use std::fs;

//Part 1 solution notes:
//Account for cycles.
//Seems like a shortest-path graph problem.
//Best path minimizes turns (as we know the input map is about 140 by 140
//so 1 additional turn would result in a bigger score for sure).
//So maybe run ahead in a straight line as much as possible, marking all nodes where you can turn (also make sure
//to mark the directions we can turn; i.e. if we can just turn right or both right and left),
//then also marking visited nodes to avoid cycles and cut paths that are longer to get to the same location.
//Need to keep 2 counters: one of steps taken, one of turns taken.
//Running ahead in a straight line as much as possible modifies BFS to minimize turns instead of steps taken.
//Also need to made sure when marking the map, to mark the minimum score to get to that location to know which path
//to cut.
//Do modified BFS with counter of turns taken and know that once 1 road ended to just exhaust
//all other queued-up potential paths with the same number of turns.
//Also note it is enough to just mark places we turn instead of every node (+ mark end node and start nodes as well).
//If we merge all nodes we get to after X turns before starting X+1 turns we can cut paths.
//Can use Hashmap with node: (turn-counter, step-counter) [instead of the score]
//and sets (instead of queues) with (node, direction to head to).
//Can use 2 sets (one we exhaust, and one where we load up (node, direction to go).

pub mod deer {
    #![allow(clippy::type_complexity)]
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };

    //When Ord is derived on structs,
    //it will produce a lexicographic ordering based on the top-to-bottom declaration order of the struct’s members.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
    struct Counter {
        turns: usize,
        steps: usize,
    }

    //This is what deriving Default does:
    // impl Default for Counter {
    //     fn default() -> Self {
    //         Counter { turns: 0, steps: 0 }
    //     }
    // }

    #[derive(Hash, PartialEq, Eq)]
    enum DeerDirection {
        North,
        South,
        West,
        East,
    }

    struct Matrix<T> {
        pub data: Vec<Vec<T>>,
        row_num: usize,
        col_num: usize,
    }

    impl<T> Matrix<T> {
        //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
        fn new(data: Vec<Vec<T>>) -> Matrix<T> {
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
    }

    pub struct DeerPatrol {
        start_location: (usize, usize),
        end_location: (usize, usize),
        map: Matrix<char>,
    }

    impl DeerPatrol {
        ///Find the start and end locations
        ///That is find indices (i,j) such that at (i,j) == 'S'
        /// and indices (w,z) such that at (w,z) == 'E'
        fn find_start_and_end(&mut self) -> Result<(), &'static str> {
            let (mut found_start, mut found_end) = (false, false);

            for (row_index, row) in self.map.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    if *char == 'S' {
                        self.start_location = (row_index, col_index);
                        found_start = true;
                    }
                    if *char == 'E' {
                        self.end_location = (row_index, col_index);
                        found_end = true;
                    }
                }
            }

            if found_start && found_end {
                Ok(())
            } else {
                Err("Could not find start location")
            }
        }

        pub fn new(map: Vec<Vec<char>>) -> DeerPatrol {
            let map = Matrix::new(map);

            let mut initial_patrol = Self {
                start_location: (0, 0),
                end_location: (0, 0),
                map,
            };

            initial_patrol.find_start_and_end().unwrap();

            initial_patrol
        }

        ///Solves the maze, returning the  
        /// lowest score a Reindeer could possibly get (the lowest path score) traversing the map
        pub fn solve_maze(&self) -> usize {
            //The Deer starts facing east
            let current_direction = DeerDirection::East;

            //At all nodes we turn we record the node location as key and Counter as value.
            //We use this to avoid traveling unecessary (bad) paths.
            let mut visited_nodes: HashMap<(usize, usize), Counter> = HashMap::new();

            visited_nodes.insert(self.start_location, Counter::default());

            let mut current_queue: HashSet<((usize, usize), DeerDirection)> = HashSet::new();

            current_queue.insert((self.start_location, current_direction));

            loop {
                //A HashSet of (node_locations, direction) to continue the search from.
                let mut next_queue: HashSet<((usize, usize), DeerDirection)> = HashSet::new();

                for (location, direction) in current_queue {
                    self.run(&mut visited_nodes, &mut next_queue, location, direction);
                }

                //Check if we hit the end location
                if let Some(final_counter) = visited_nodes.get(&self.end_location) {
                    let score = final_counter.turns * 1000 + final_counter.steps;
                    break score;
                    //Note if taking 1 additional turn means finding more paths to the end location,
                    //those paths will definitely have a worse score so there is no point to keep going.
                }

                //If we did not hit the end location, check the next queue (meaning we take 1 additional turn).
                current_queue = next_queue;
            }
        }

        ///Runs in a straight line starting from location in direction, marking every node we can turn or the end node
        /// with the current Counter (turns_counter and steps_counter).
        fn run(
            &self,
            visited_nodes: &mut HashMap<(usize, usize), Counter>,
            next_queue: &mut HashSet<((usize, usize), DeerDirection)>,
            mut location: (usize, usize),
            direction: DeerDirection,
        ) {
            let mut counter = *visited_nodes.get(&location).unwrap();

            //We do things this way to guarantee the following:
            //All entries in current_queue have taken X turns and all entries in next_queue have taken X+1 turns.
            if self.start_location == location {
                if direction == DeerDirection::East {
                    next_queue.insert((self.start_location, DeerDirection::North));
                    next_queue.insert((self.start_location, DeerDirection::South));
                } else {
                    //Meaning we already turned North or South or West.
                    //Just for the starting location-- this means we have to fix the counter
                    if direction == DeerDirection::North || direction == DeerDirection::South {
                        counter.turns = 1;

                        next_queue.insert((self.start_location, DeerDirection::West));
                    } else if direction == DeerDirection::West {
                        counter.turns = 2;
                    }
                }
            }

            match direction {
                DeerDirection::East => {
                    while self.map.data[location.0][location.1 + 1] != '#' {
                        location.1 += 1;
                        counter.steps += 1;

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|final_counter| {
                                    if counter < *final_counter {
                                        *final_counter = counter;
                                    }
                                })
                                .or_insert(counter);
                            return;
                        } else if self.map.data[location.0 + 1][location.1] != '#'
                            || self.map.data[location.0 - 1][location.1] != '#'
                        {
                            //meaning we can turn North or South

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            if old.is_none_or(|old_counter| *old_counter > new_counter) {
                                visited_nodes.insert(location, new_counter);

                                //it is ok to enqueue both North and South even if the deer
                                //can only head in 1 of these directions as the first thing
                                //we check in the while loop is that we are not stepping into a wall
                                next_queue.insert((location, DeerDirection::North));
                                next_queue.insert((location, DeerDirection::South));
                            }
                        }
                    }
                }
                DeerDirection::West => {
                    while self.map.data[location.0][location.1 - 1] != '#' {
                        location.1 -= 1;
                        counter.steps += 1;

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|final_counter| {
                                    if counter < *final_counter {
                                        *final_counter = counter;
                                    }
                                })
                                .or_insert(counter);
                            return;
                        } else if self.map.data[location.0 + 1][location.1] != '#'
                            || self.map.data[location.0 - 1][location.1] != '#'
                        {
                            //meaning we can turn North or South

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            if old.is_none_or(|old_counter| *old_counter > new_counter) {
                                visited_nodes.insert(location, new_counter);

                                next_queue.insert((location, DeerDirection::North));
                                next_queue.insert((location, DeerDirection::South));
                            }
                        }
                    }
                }
                DeerDirection::North => {
                    while self.map.data[location.0 - 1][location.1] != '#' {
                        location.0 -= 1;
                        counter.steps += 1;

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|final_counter| {
                                    if counter < *final_counter {
                                        *final_counter = counter;
                                    }
                                })
                                .or_insert(counter);
                            return;
                        } else if self.map.data[location.0][location.1 + 1] != '#'
                            || self.map.data[location.0][location.1 - 1] != '#'
                        {
                            //meaning we can turn East or West

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            if old.is_none_or(|old_counter| *old_counter > new_counter) {
                                visited_nodes.insert(location, new_counter);

                                next_queue.insert((location, DeerDirection::East));
                                next_queue.insert((location, DeerDirection::West));
                            }
                        }
                    }
                }
                DeerDirection::South => {
                    while self.map.data[location.0 + 1][location.1] != '#' {
                        location.0 += 1;
                        counter.steps += 1;

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|final_counter| {
                                    if counter < *final_counter {
                                        *final_counter = counter;
                                    }
                                })
                                .or_insert(counter);
                            return;
                        } else if self.map.data[location.0][location.1 + 1] != '#'
                            || self.map.data[location.0][location.1 - 1] != '#'
                        {
                            //meaning we can turn East or West

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            if old.is_none_or(|old_counter| *old_counter > new_counter) {
                                visited_nodes.insert(location, new_counter);

                                next_queue.insert((location, DeerDirection::East));
                                next_queue.insert((location, DeerDirection::West));
                            }
                        }
                    }
                }
            }
        }
    }

    //Part 2 stuff

    impl DeerPatrol {
        ///Solves the maze, returning how many tiles are part of at least one of the best paths through the maze
        pub fn solve_maze_with_trace(&self) -> usize {
            //The Deer starts facing east
            let current_direction = DeerDirection::East;

            //At all nodes we turn we record the node location as key and Counter as value.
            //We use this to avoid traveling unecessary (bad) paths.

            //Part 2: We also record the HashSet of **all** best paths leading to that node.
            let mut visited_nodes: HashMap<(usize, usize), (Counter, HashSet<(usize, usize)>)> =
                HashMap::new();

            //We know the start location will be a part of all best paths
            let mut initial = HashSet::new();
            initial.insert(self.start_location);

            visited_nodes.insert(self.start_location, (Counter::default(), initial));

            let mut current_queue: HashSet<((usize, usize), DeerDirection)> = HashSet::new();

            current_queue.insert((self.start_location, current_direction));

            loop {
                //A HashSet of (node_locations, direction) to continue the search from.
                let mut next_queue: HashSet<((usize, usize), DeerDirection)> = HashSet::new();

                for (location, direction) in current_queue {
                    self.run_with_trace(&mut visited_nodes, &mut next_queue, location, direction);
                }

                //Check if we hit the end location
                if let Some((_final_counter, best_paths)) = visited_nodes.get(&self.end_location) {
                    //let score = final_counter.turns *1000 +final_counter.steps;
                    //break score;

                    break best_paths.iter().count();

                    //Note if taking 1 additional turn means finding more paths to the end location,
                    //those paths will definitely have a worse score so there is no point to keep going.
                }

                //If we did not hit the end location, check the next queue (meaning we take 1 additional turn).
                current_queue = next_queue;
            }
        }

        ///Runs in a straight line starting from location in direction, marking every node we can turn or the end node
        /// with the current Counter (turns_counter and steps_counter).
        /// Part 2: Also keeps track of the HashSets of the best paths leading to the turning locations
        /// and the end location.
        fn run_with_trace(
            &self,
            visited_nodes: &mut HashMap<(usize, usize), (Counter, HashSet<(usize, usize)>)>,
            next_queue: &mut HashSet<((usize, usize), DeerDirection)>,
            mut location: (usize, usize),
            direction: DeerDirection,
        ) {
            let (mut counter, mut best_paths) = visited_nodes.get(&location).unwrap().clone();

            if self.start_location == location {
                if direction == DeerDirection::East {
                    next_queue.insert((self.start_location, DeerDirection::North));
                    next_queue.insert((self.start_location, DeerDirection::South));
                } else {
                    //Meaning we already turned North or South or West.
                    //Just for the starting location-- this means we have to fix the counter
                    if direction == DeerDirection::North || direction == DeerDirection::South {
                        counter.turns = 1;

                        next_queue.insert((self.start_location, DeerDirection::West));
                    } else if direction == DeerDirection::West {
                        counter.turns = 2;
                    }
                }
            }

            match direction {
                DeerDirection::East => {
                    while self.map.data[location.0][location.1 + 1] != '#' {
                        location.1 += 1;
                        counter.steps += 1;

                        //Part 2: add location to best paths
                        best_paths.insert(location);

                        let mut best_paths_copy = best_paths.clone();

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|(final_counter, current_best)| {
                                    match counter.cmp(final_counter) {
                                        Ordering::Less => {
                                            *final_counter = counter;
                                            *current_best = best_paths_copy;
                                        }
                                        Ordering::Equal => {
                                            //add best_paths to current_best
                                            current_best.extend(best_paths_copy);
                                        }
                                        _ => (),
                                    }
                                })
                                .or_insert((counter, best_paths));
                            return;
                        } else if self.map.data[location.0 + 1][location.1] != '#'
                            || self.map.data[location.0 - 1][location.1] != '#'
                        {
                            //meaning we can turn North or South

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            match old {
                                Some((old_counter, old_best_paths)) => {
                                    match new_counter.cmp(old_counter) {
                                        Ordering::Less => {
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Equal => {
                                            best_paths_copy.extend(old_best_paths);
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Greater => (),
                                    }

                                    //Note we made sure our queue is consistent with X turns.
                                    //That is all entries on current_queue took X turns and all entries in next_queue
                                    //have taken X+1 turns.
                                    //So if *old_counter >= new_counter (i.e. new_counter took less turns
                                    //to get here [which is not possible] or less steps to get here), then
                                    //we know for a fact next_queue already has the following enqueued:
                                    // next_queue.insert((location, DeerDirection::North));
                                    // next_queue.insert((location, DeerDirection::South));
                                }
                                None => {
                                    visited_nodes.insert(location, (new_counter, best_paths_copy));

                                    //it is ok to enqueue both North and South even if the deer
                                    //can only head in 1 of these directions as the first thing
                                    //we check in the while loop is that we are not stepping into a wall
                                    next_queue.insert((location, DeerDirection::North));
                                    next_queue.insert((location, DeerDirection::South));
                                }
                            }
                        }
                    }
                }
                DeerDirection::West => {
                    while self.map.data[location.0][location.1 - 1] != '#' {
                        location.1 -= 1;
                        counter.steps += 1;

                        //Part 2: add location to best paths
                        best_paths.insert(location);

                        let mut best_paths_copy = best_paths.clone();

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|(final_counter, current_best)| {
                                    match counter.cmp(final_counter) {
                                        Ordering::Less => {
                                            *final_counter = counter;
                                            *current_best = best_paths_copy;
                                        }
                                        Ordering::Equal => {
                                            //add best_paths to current_best
                                            current_best.extend(best_paths_copy);
                                        }
                                        _ => (),
                                    }
                                })
                                .or_insert((counter, best_paths));
                            return;
                        } else if self.map.data[location.0 + 1][location.1] != '#'
                            || self.map.data[location.0 - 1][location.1] != '#'
                        {
                            //meaning we can turn North or South

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            match old {
                                Some((old_counter, old_best_paths)) => {
                                    match new_counter.cmp(old_counter) {
                                        Ordering::Less => {
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Equal => {
                                            best_paths_copy.extend(old_best_paths);
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Greater => (),
                                    }
                                }
                                None => {
                                    visited_nodes.insert(location, (new_counter, best_paths_copy));

                                    next_queue.insert((location, DeerDirection::North));
                                    next_queue.insert((location, DeerDirection::South));
                                }
                            }
                        }
                    }
                }

                DeerDirection::North => {
                    while self.map.data[location.0 - 1][location.1] != '#' {
                        location.0 -= 1;
                        counter.steps += 1;

                        //Part 2: add location to best paths
                        best_paths.insert(location);

                        let mut best_paths_copy = best_paths.clone();

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|(final_counter, current_best)| {
                                    match counter.cmp(final_counter) {
                                        Ordering::Less => {
                                            *final_counter = counter;
                                            *current_best = best_paths_copy;
                                        }
                                        Ordering::Equal => {
                                            //add best_paths to current_best
                                            current_best.extend(best_paths_copy);
                                        }
                                        _ => (),
                                    }
                                })
                                .or_insert((counter, best_paths));
                            return;
                        } else if self.map.data[location.0][location.1 + 1] != '#'
                            || self.map.data[location.0][location.1 - 1] != '#'
                        {
                            //meaning we can turn East or West

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            match old {
                                Some((old_counter, old_best_paths)) => {
                                    match new_counter.cmp(old_counter) {
                                        Ordering::Less => {
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Equal => {
                                            best_paths_copy.extend(old_best_paths);
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Greater => (),
                                    }
                                }
                                None => {
                                    visited_nodes.insert(location, (new_counter, best_paths_copy));

                                    next_queue.insert((location, DeerDirection::East));
                                    next_queue.insert((location, DeerDirection::West));
                                }
                            }
                        }
                    }
                }
                DeerDirection::South => {
                    while self.map.data[location.0 + 1][location.1] != '#' {
                        location.0 += 1;
                        counter.steps += 1;

                        //Part 2: add location to best paths
                        best_paths.insert(location);

                        let mut best_paths_copy = best_paths.clone();

                        if self.map.data[location.0][location.1] == 'E' {
                            visited_nodes
                                .entry(location)
                                .and_modify(|(final_counter, current_best)| {
                                    match counter.cmp(final_counter) {
                                        Ordering::Less => {
                                            *final_counter = counter;
                                            *current_best = best_paths_copy;
                                        }
                                        Ordering::Equal => {
                                            //add best_paths to current_best
                                            current_best.extend(best_paths_copy);
                                        }
                                        _ => (),
                                    }
                                })
                                .or_insert((counter, best_paths));
                            return;
                        } else if self.map.data[location.0][location.1 + 1] != '#'
                            || self.map.data[location.0][location.1 - 1] != '#'
                        {
                            //meaning we can turn East or West

                            let old = visited_nodes.get(&location);

                            let new_counter = Counter {
                                turns: counter.turns + 1,
                                steps: counter.steps,
                            };

                            match old {
                                Some((old_counter, old_best_paths)) => {
                                    match new_counter.cmp(old_counter) {
                                        Ordering::Less => {
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Equal => {
                                            best_paths_copy.extend(old_best_paths);
                                            visited_nodes
                                                .insert(location, (new_counter, best_paths_copy));
                                        }
                                        Ordering::Greater => (),
                                    }
                                }
                                None => {
                                    visited_nodes.insert(location, (new_counter, best_paths_copy));

                                    next_queue.insert((location, DeerDirection::East));
                                    next_queue.insert((location, DeerDirection::West));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

///Returns the lowest score a Reindeer could possibly get (the lowest path score) traversing the map
fn solution_part1(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let patrol = deer::DeerPatrol::new(data);

    patrol.solve_maze()
}

//Part 2 solution notes: Similar to part 1 but this time have an additional HashSet accompaning all travel legs
//(if strictly better then replace the current HashSet, if just as good then merge with current HashSet, if
//worse do nothing).

///Returns how many tiles are part of at least one of the best paths through the maze
fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let patrol = deer::DeerPatrol::new(data);

    patrol.solve_maze_with_trace()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day16.txt"));
        dbg!(solution_part2("puzzle_inputs/day16.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day16example.txt");
        assert_eq!(result, 11048);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day16example.txt");
        assert_eq!(result, 64);
    }
}
