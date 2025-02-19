use std::{collections::HashMap, fs};

use race::Program;

//Part 1 solution notes:
//Notice a cheat enables to program (the race runner) to phase through exactly one '#'.
//Also note a program is only allowed to cheat once during a race.

//The key to this problem is that "there is only a single path from the start to the end".
//We can also solve the problem if this was not true in a similar way to what we do below,
//but this just makes things simpler.

//This seems like a slightly modified shortest-path problem.
//What we want to know is for every cheat-- how many picoseconds (each picosecond is one step)
//does it shave off the shortest-path (which is also the only path) with no cheats.
//So in particular we want to solve the race with no cheats.

//So solve the maze with no cheats and keep a HashMap of all locations the NoCheat-path visited
//with key: location and value: the number of step it is in the NoCheat path.
//Then iterate on that HashMap and see if we can save time by cheating and if so how much.

//Note:
//There can be something like:
// ## # # #
// #S . . #     <-- only one path that goes from S to E
// ## # . #
// #. # . #  <-- but there is a valid shortcut from S going down
// #E # . #
// #. # . #
// #. . . #
// ## # # #
//What we need to do is also run BFS starting at the end location.
//We keep track of all locations that can be reached and in how many steps.

//Part 2 solution notes:
//Now cheats *can* last up to 20 picoseconds steps.
//So for each location on the NoCheat path (that is the starting location of the cheat):
//Need to find a list of all *unique* end locations of possible cheats that have length up to 20.
//For each end location, we also want to get the time saved by this cheat.
//In fact, we only care about the time saved for each cheat.

//An end location is a non-'#' location hit (note that there might be longer cheats
//that go through that location).

pub mod race {
    use std::{
        collections::{HashMap, VecDeque},
        mem,
    };

    struct Matrix<T> {
        data: Vec<Vec<T>>,
        row_num: usize,
        col_num: usize,
    }

    impl<T> Matrix<T> {
        //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
        fn build(data: Vec<Vec<T>>) -> Matrix<T> {
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

    struct NoCheatRunner {
        step: usize,
        ///location: which step number was it
        record: HashMap<(usize, usize), usize>,
        curr_location: (usize, usize),
    }

    impl NoCheatRunner {
        fn new(start_location: (usize, usize)) -> NoCheatRunner {
            NoCheatRunner {
                step: 0,
                record: HashMap::new(),
                curr_location: start_location,
            }
        }
    }

    pub struct Program {
        maze: Matrix<char>,
        race_start: (usize, usize),
        race_end: (usize, usize),
    }

    impl Program {
        pub fn build(maze: Vec<Vec<char>>) -> Program {
            let maze = Matrix::build(maze);
            let [race_start, race_end] = Program::find_start_and_end(&maze).unwrap();

            Program {
                maze,
                race_start,
                race_end,
            }
        }

        fn find_start_and_end(maze: &Matrix<char>) -> Result<[(usize, usize); 2], &'static str> {
            let mut start = None;
            let mut end = None;

            for (row_index, row) in maze.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    match *char {
                        'S' => {
                            start = Some((row_index, col_index));
                        }
                        'E' => {
                            end = Some((row_index, col_index));
                        }
                        _ => (),
                    }
                }
            }

            match (start, end) {
                (Some(start), Some(end)) => Ok([start, end]),
                _ => Err("Could not find start or end"),
            }
        }

        ///Returns a Hash map with key being number of picoseconds saved and value
        /// being the number of cheats that save the program that many seconds.
        pub fn find_cheats(&self) -> HashMap<usize, usize> {
            let no_cheat_path: HashMap<(usize, usize), usize> = self.solve_without_cheats();

            let end_step = *no_cheat_path
                .get(&self.race_end)
                .expect("The path should contain the race_end");

            //We now iterate on the no_cheat path and see if we can find any shortcuts,
            //if we find a shortcut we add it to the time_saved_counter.

            //Note shortcuts don't have to end up on another location in the no_cheat path (see
            //solve_end_to_start docs).

            let dist_from_end = self.solve_end_to_start();

            //The key is the number of picoseconds saved and the value is
            // the number of cheats that save the program that many seconds.
            let mut time_saved_counter: HashMap<usize, usize> = HashMap::new();

            for (loc, cur_step) in no_cheat_path {
                //Test this location for shortcuts

                //Test for a shortcut going up
                //(meaning 1 space up is a '#' and 2 spaces up is in dist_from_end)
                if loc.0 > 1 {
                    if let Some(&step_from_end) = dist_from_end.get(&(loc.0 - 2, loc.1))
                    //Note we check two spaces up!
                    {
                        //We are guaranteed step_from_end is <= end_step by the implementation of solve_end_to_start.
                        let next_step = end_step - step_from_end;

                        //This means we didn't just go up twice (i.e it is not the case that 1 space up is '.')
                        if next_step > cur_step + 2 {
                            //We found a shortcut
                            let time_saved = next_step - (cur_step + 2);
                            time_saved_counter
                                .entry(time_saved)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                    }
                }

                //Test for a shorcut going down
                if let Some(&step_from_end) = dist_from_end.get(&(loc.0 + 2, loc.1)) {
                    let next_step = end_step - step_from_end;

                    if next_step > cur_step + 2 {
                        let time_saved = next_step - (cur_step + 2);
                        time_saved_counter
                            .entry(time_saved)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }

                //Test for a shorcut going left
                if loc.1 > 1 {
                    if let Some(&step_from_end) = dist_from_end.get(&(loc.0, loc.1 - 2)) {
                        let next_step = end_step - step_from_end;

                        if next_step > cur_step + 2 {
                            let time_saved = next_step - (cur_step + 2);
                            time_saved_counter
                                .entry(time_saved)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                    }
                }

                //Test for a shorcut going right
                if let Some(&step_from_end) = dist_from_end.get(&(loc.0, loc.1 + 2)) {
                    let next_step = end_step - step_from_end;

                    if next_step > cur_step + 2 {
                        let time_saved = next_step - (cur_step + 2);
                        time_saved_counter
                            .entry(time_saved)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
            }

            time_saved_counter
        }

        ///we solve the maze (start to end) using BFS (Breadth-First Search) once without cheats.
        ///Recall there is only one NoCheat path from the start to the end.
        ///Returns a HashMap with key: location and value: the number of step it is in the NoCheat path.
        fn solve_without_cheats(&self) -> HashMap<(usize, usize), usize> {
            //The important thing to note is that even though there is only 1 NoCheat path
            //from the start to the end, there might be multiple "dead-end" paths along the way.

            //This is why we use multiple NoCheat runners
            let runner = NoCheatRunner::new(self.race_start);

            //A vector of runners to advance by 1 step at a time
            let mut runners = VecDeque::new();
            runners.push_back(runner);

            while let Some(mut runner) = runners.pop_front() {
                //first we update the runners record
                runner.record.insert(runner.curr_location, runner.step);
                runner.step += 1;

                //once a NoCheat runner has reached the end we know we are done
                if runner.curr_location == self.race_end {
                    return runner.record;
                }

                //See if the runner can go up
                //(meaning the runner stays within the map and there is no obstacle there)
                //and that we have yet to visit that location
                if runner.curr_location.0 > 0
                    && self.maze.data[runner.curr_location.0 - 1][runner.curr_location.1] != '#'
                    && !runner
                        .record
                        .contains_key(&(runner.curr_location.0 - 1, runner.curr_location.1))
                {
                    let new_runner = NoCheatRunner {
                        step: runner.step,
                        record: runner.record.clone(),
                        curr_location: (runner.curr_location.0 - 1, runner.curr_location.1),
                    };
                    runners.push_back(new_runner);
                }

                //See if the runner can go down
                if runner.curr_location.0 + 1 < self.maze.row_num
                    && self.maze.data[runner.curr_location.0 + 1][runner.curr_location.1] != '#'
                    && !runner
                        .record
                        .contains_key(&(runner.curr_location.0 + 1, runner.curr_location.1))
                {
                    let new_runner = NoCheatRunner {
                        step: runner.step,
                        record: runner.record.clone(),
                        curr_location: (runner.curr_location.0 + 1, runner.curr_location.1),
                    };
                    runners.push_back(new_runner);
                }

                //See if the runner can go left
                if runner.curr_location.1 > 0
                    && self.maze.data[runner.curr_location.0][runner.curr_location.1 - 1] != '#'
                    && !runner
                        .record
                        .contains_key(&(runner.curr_location.0, runner.curr_location.1 - 1))
                {
                    let new_runner = NoCheatRunner {
                        step: runner.step,
                        record: runner.record.clone(),
                        curr_location: (runner.curr_location.0, runner.curr_location.1 - 1),
                    };
                    runners.push_back(new_runner);
                }

                //See if the runner can go right
                if runner.curr_location.1 + 1 < self.maze.col_num
                    && self.maze.data[runner.curr_location.0][runner.curr_location.1 + 1] != '#'
                    && !runner
                        .record
                        .contains_key(&(runner.curr_location.0, runner.curr_location.1 + 1))
                {
                    //since this is the last case, we can avoid having to clone runner.record.
                    let new_runner = NoCheatRunner {
                        step: runner.step,
                        record: runner.record,
                        curr_location: (runner.curr_location.0, runner.curr_location.1 + 1),
                    };
                    runners.push_back(new_runner);
                }
            }

            //If we got here that means there is no solution to the maze. This is impossible!
            panic!("No solution (without cheating) to the maze")
        }

        ///Note:
        ///There can be something like:
        ///
        ///             ## # # #
        ///             #S . . #     <-- only one path that goes from S to E
        ///             ## # . #
        ///             #. # . #  <-- but there is a valid shortcut from S going down
        ///             #E # . #
        ///             #. # . #
        ///             #. . . #
        ///             ## # # #
        ///
        ///What we need to do is also run BFS starting at the end location.
        ///We keep track of all locations that can be reached and in how many steps (from the end).
        ///
        ///Actually, it is enough to just keep track of all locations that can be reached
        /// in at most (steps from start to end) -1 steps. Then if a shortcut
        /// will end up on somewhere not in this HashMap then we know it is at:
        /// * an unreachable location from the end
        /// * a location that takes longer to reach the end then if we have not taken the "shortcut"
        ///
        /// Either way, it won't be a shortcut
        fn solve_end_to_start(&self) -> HashMap<(usize, usize), usize> {
            let mut end_record = HashMap::new();
            end_record.insert(self.race_end, 0);

            let mut steps = 1;
            let mut visit_next = vec![self.race_end];

            'outer: loop {
                let mut visit_now = vec![];
                mem::swap(&mut visit_now, &mut visit_next);

                while let Some(cur_loc) = visit_now.pop() {
                    if cur_loc == self.race_start {
                        break 'outer;
                    }

                    //go up
                    if cur_loc.0 > 0
                        && self.maze.data[cur_loc.0 - 1][cur_loc.1] != '#'
                        && !end_record.contains_key(&(cur_loc.0 - 1, cur_loc.1))
                    {
                        end_record.insert((cur_loc.0 - 1, cur_loc.1), steps);
                        visit_next.push((cur_loc.0 - 1, cur_loc.1));
                    }

                    //go down
                    if cur_loc.0 + 1 < self.maze.row_num
                        && self.maze.data[cur_loc.0 + 1][cur_loc.1] != '#'
                        && !end_record.contains_key(&(cur_loc.0 + 1, cur_loc.1))
                    {
                        end_record.insert((cur_loc.0 + 1, cur_loc.1), steps);
                        visit_next.push((cur_loc.0 + 1, cur_loc.1));
                    }

                    //go left
                    if cur_loc.1 > 0
                        && self.maze.data[cur_loc.0][cur_loc.1 - 1] != '#'
                        && !end_record.contains_key(&(cur_loc.0, cur_loc.1 - 1))
                    {
                        end_record.insert((cur_loc.0, cur_loc.1 - 1), steps);
                        visit_next.push((cur_loc.0, cur_loc.1 - 1));
                    }

                    //go right
                    if cur_loc.1 + 1 < self.maze.col_num
                        && self.maze.data[cur_loc.0][cur_loc.1 + 1] != '#'
                        && !end_record.contains_key(&(cur_loc.0, cur_loc.1 + 1))
                    {
                        end_record.insert((cur_loc.0, cur_loc.1 + 1), steps);
                        visit_next.push((cur_loc.0, cur_loc.1 + 1));
                    }
                }
                steps += 1;
            }

            end_record
        }

        //Part_2 specific stuff

        ///Returns a Hash map with key being number of picoseconds saved and value
        /// being the number of cheats (that can last at most 20 seconds) that save the program that many seconds.
        pub fn find_extended_cheats(&self) -> HashMap<usize, usize> {
            let no_cheat_path: HashMap<(usize, usize), usize> = self.solve_without_cheats();

            let dist_from_end = self.solve_end_to_start();
            let end_step = *no_cheat_path
                .get(&self.race_end)
                .expect("The path should contain the race_end");

            //We now iterate on the no_cheat path and see if we can find any shortcuts,
            //if we find a shortcut we add it to the time_saved_counter

            //The key is the number of picoseconds saved and the value is
            // the number of cheats that save the program that many seconds.
            let mut time_saved_counter: HashMap<usize, usize> = HashMap::new();

            for (loc, start_step) in no_cheat_path {
                self.find_extended_shortcuts(
                    loc,
                    start_step,
                    end_step,
                    &dist_from_end,
                    &mut time_saved_counter,
                );
            }

            time_saved_counter
        }

        ///Find all cheats that start at start_loc and last at most 20 steps.
        /// Record the findings in time_saved_counter
        fn find_extended_shortcuts(
            &self,
            start_loc: (usize, usize),
            start_step: usize,
            end_step: usize,
            dist_from_end: &HashMap<(usize, usize), usize>,
            time_saved_counter: &mut HashMap<usize, usize>,
        ) {
            //We want to iterate over all locations that have L1 distance
            // a.k.a. Manhattan distance (a.k.a. taxi-cab distance)
            // of at most 20 from the starting location
            let loc_dist = self.calc_taxi_dist(start_loc);

            //loc_dist now contains all possible end-locations for a cheat that starts at this start location.
            //we iterate on it and if that end-location is a shortcut then we add it to time_saved_counter

            for (end_loc, cheat_len) in loc_dist {
                if let Some(&step_from_end) = dist_from_end.get(&end_loc) {
                    //We are guaranteed step_from_end is <= end_step by the implementation of solve_end_to_start.
                    let next_step = end_step - step_from_end;

                    if next_step > start_step + cheat_len {
                        let time_saved = next_step - (start_step + cheat_len);
                        time_saved_counter
                            .entry(time_saved)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        ///Returns a vector of locations in the program map
        /// that have taxi-cab distance at most 20 from loc and their distance (the cheat length)
        fn calc_taxi_dist(&self, loc: (usize, usize)) -> Vec<((usize, usize), usize)> {
            let mut loc_dist = vec![];

            'up: for vertical_dist in 0..=20 {
                //See if we can go up
                if loc.0 >= vertical_dist {
                    'right: for horizontal_dist in 0..=(20 - vertical_dist) {
                        //See if we can go right
                        if loc.1 + horizontal_dist < self.maze.col_num {
                            let current_location = (loc.0 - vertical_dist, loc.1 + horizontal_dist);
                            loc_dist.push((current_location, vertical_dist + horizontal_dist));
                        } else {
                            break 'right;
                        }
                    }

                    //Note we start at 1 so we don't double count places that have horizontal distance 0
                    'left: for horizontal_dist in 1..=(20 - vertical_dist) {
                        //See if we can go left
                        if loc.1 >= horizontal_dist {
                            let current_location = (loc.0 - vertical_dist, loc.1 - horizontal_dist);
                            loc_dist.push((current_location, vertical_dist + horizontal_dist));
                        } else {
                            break 'left;
                        }
                    }
                } else {
                    break 'up;
                }
            }

            //Note we start at 1 so we don't double count places that have vertical distance 0
            'down: for vertical_dist in 1..=20 {
                //See if we can go down
                if loc.0 + vertical_dist < self.maze.row_num {
                    'right: for horizontal_dist in 0..=(20 - vertical_dist) {
                        //See if we can go right
                        if loc.1 + horizontal_dist < self.maze.col_num {
                            let current_location = (loc.0 + vertical_dist, loc.1 + horizontal_dist);
                            loc_dist.push((current_location, vertical_dist + horizontal_dist));
                        } else {
                            break 'right;
                        }
                    }

                    //Note we start at 1 so we don't double count places that have horizontal distance 0
                    'left: for horizontal_dist in 1..=(20 - vertical_dist) {
                        //See if we can go left
                        if loc.1 >= horizontal_dist {
                            let current_location = (loc.0 + vertical_dist, loc.1 - horizontal_dist);
                            loc_dist.push((current_location, vertical_dist + horizontal_dist));
                        } else {
                            break 'left;
                        }
                    }
                } else {
                    break 'down;
                }
            }

            loc_dist
        }
    }
}

///Returns a Hash map with key being number of picoseconds saved and value
/// being the number of cheats that save the program that many seconds.
/// We then can extract later how many cheats would save the program at least 100 picoseconds
fn solution_part1(file_path: &str) -> HashMap<usize, usize> {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let program = Program::build(data);
    program.find_cheats()
}

/// Returns how many cheats would save you at least 100 picoseconds (now that cheats can last at most 20 picoseconds)
fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let program = Program::build(data);
    program
        .find_extended_cheats()
        .into_iter()
        .fold(0, |mut acc: usize, (key, value)| {
            if key >= 100 {
                acc += value;
            }
            acc
        })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        let result = solution_part1("puzzle_inputs/day20.txt");
        //The answer we need as how many cheats would save us at least 100 seconds
        let answer = result.into_iter().fold(0, |mut acc: usize, (key, value)| {
            if key >= 100 {
                acc += value;
            }
            acc
        });
        dbg!(answer);
        dbg!(solution_part2("puzzle_inputs/day20.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day20example.txt");
        let mut example_sol: HashMap<usize, usize> = HashMap::new();

        //The key is the number of picoseconds saved and the value is
        // the number of cheats that save the program that many seconds.
        example_sol.insert(2, 14);
        example_sol.insert(4, 14);
        example_sol.insert(6, 2);
        example_sol.insert(8, 4);
        example_sol.insert(10, 2);
        example_sol.insert(12, 3);
        example_sol.insert(20, 1);
        example_sol.insert(36, 1);
        example_sol.insert(38, 1);
        example_sol.insert(40, 1);
        example_sol.insert(64, 1);

        assert_eq!(result, example_sol);
    }

    #[test]
    fn example_part2() {
        let data: Vec<_> = fs::read_to_string("puzzle_inputs/day20example.txt")
            .expect("failed to open file")
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let program = Program::build(data);
        let mut filtered: Vec<(usize, usize)> = program
            .find_extended_cheats()
            .into_iter()
            .filter(|&(key, _)| key >= 50)
            .collect();

        filtered.sort();

        dbg!(&filtered);
    }
}
