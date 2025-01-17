use std::fs;

use regex::Regex;
use security::Robot;

//Part 1 solution notes:
//We note we don't actually need to plot the robots on the room.
//We can simply use a Vector of Robots and update the values (the positions of the robots).
//Note this is simple modulo arithmetic so we can update the values all at once (since x-axis movement
//is independent of y-axis movement).
//So one function to update the robots positions after X seconds have elapsed (X is part of the input).
//And one function to calculate the safety factor.

pub mod security {

    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };

    pub struct Robot {
        position: (usize, usize),
        velocity: (i32, i32),
    }
    impl Robot {
        pub fn new(position: (usize, usize), velocity: (i32, i32)) -> Robot {
            Robot { position, velocity }
        }
    }

    pub struct Room {
        robots: Vec<Robot>,
        width: usize,
        height: usize,
    }

    impl Room {
        pub fn new(robots: Vec<Robot>, width: usize, height: usize) -> Room {
            Room {
                robots,
                width,
                height,
            }
        }

        ///Make X seconds elapse and update all the Robots positions accordingly
        pub fn elapse_time(&mut self, seconds: usize) {
            //From the docs on rem_euclid(self, rhs: i32) -> i32:
            //Calculates the least nonnegative remainder of self (mod rhs)

            for robot in &mut self.robots {
                robot.position.0 = ((robot.position.0 as i32) + (seconds as i32) * robot.velocity.0)
                    .rem_euclid(self.width as i32) as usize;
                robot.position.1 = ((robot.position.1 as i32) + (seconds as i32) * robot.velocity.1)
                    .rem_euclid(self.height as i32) as usize;
            }
        }

        ///Calculates the Room's current saftey factor.
        /// That is calculate and multiply the number of Robots in each quadrant.
        /// Remember Robots that are exactly in the middle (horizontally or vertically)
        /// don't count as being in any quadrant.
        pub fn safety_factor(&self) -> usize {
            //Remember in Rust  / rounds down
            let mid_col = self.width / 2;
            let mid_row = self.height / 2;

            let mut count_quad: [usize; 4] = [0; 4];

            for robot in &self.robots {
                if robot.position.0 == mid_col || robot.position.1 == mid_row {
                    continue;
                }
                match (
                    robot.position.0.cmp(&mid_col),
                    robot.position.1.cmp(&mid_row),
                ) {
                    (Ordering::Equal, _) | (_, Ordering::Equal) => continue,
                    (Ordering::Less, Ordering::Less) => {
                        count_quad[0] += 1;
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        count_quad[1] += 1;
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        count_quad[2] += 1;
                    }
                    (Ordering::Greater, Ordering::Greater) => {
                        count_quad[3] += 1;
                    }
                }
            }

            count_quad.into_iter().product()
        }

        ///Checks if the robots are in a tree shape by printing to the screen as
        /// there was no indication of whether the tree had a trunk, or was centered, or was fethered
        /// (meaning the number of robots per row can decrease potentially as we go down the tree),
        /// or was filled in or not.
        /// We can make our inspection faster by only printing if we have a column
        /// with 20 or more robots (value picked by trial and error).
        /// Overall, this was not my favorite puzzle.
        ///
        ///
        /// The Tree ended up looking like this (0 is where a robot is, a . is a space with no robot; this is only part
        /// of the room):
        ///
        ///
        ///
        ///     .............0.......................................................................................
        ///     .....................................................................................................
        ///     ................................0000000000000000000000000000000......................................
        ///     ....................0...........0.............................0......................................
        ///     ................................0.............................0...................................00.
        ///     ................................0.............................0............................0.........
        ///     ................................0.............................0......................................
        ///     ................0...0...........0..............0..............0.....................0................
        ///     ................................0.............000.............0......................................
        ///     ..........................0.....0............00000............0......0...............................
        ///     ...........................0....0...........0000000...........0....0.................................
        ///     ..........0.....................0..........000000000..........0..................0.....0.............
        ///     ................................0............00000............0......................................
        ///     ................................0...........0000000...........0.......0.....0........................
        ///     ................................0..........000000000..........0......................................
        ///     ................................0.........00000000000.........0.....................0................
        ///     .........0.............0........0........0000000000000........0......................................
        ///     ..........................0.....0..........000000000..........0......................................
        ///     ..........0.....................0.........00000000000.........0......................................
        ///     0...............................0........0000000000000........0......................................
        ///     .........................0......0.......000000000000000.......0.........0............................
        ///     ................................0......00000000000000000......0............0.........................
        ///     ................................0........0000000000000........0.....................................0
        ///     .............................0..0.......000000000000000.......0........................0.............
        ///     ............0...................0......00000000000000000......0......................................
        ///     .......0........................0.....0000000000000000000.....0......................................
        ///     .................0..............0....000000000000000000000....0......................................
        ///     ................................0.............000.............0......................................
        ///     ................................0.............000.............0.....................0................
        ///     0...............................0.............000.............0...............0......................
        ///     ...........................00...0.............................0..............................0.......
        ///     ................................0.............................0......................................
        ///     ................................0.............................0..........0...........................
        ///     ................................0000000000000000000000000000000......................................
        ///     ...0....0............................................................................................
        ///
        ///
        ///
        ///
        pub fn check_tree(&self, count: usize) {
            //We use a HashSet to eliminate duplicates
            let robot_positions: HashSet<(usize, usize)> =
                self.robots.iter().map(|robot| robot.position).collect();

            let mut column_counts: HashMap<usize, usize> = HashMap::new();

            for position in robot_positions.iter() {
                column_counts
                    .entry(position.0)
                    .and_modify(|e| *e += 1)
                    .or_insert(0);
            }

            if *column_counts.values().max().unwrap() >= 20 {
                println!("The number of seconds is: {} and the tree is: \n", count);

                for y in 0..self.height {
                    println!("\n"); //start a new line
                    for x in 0..self.width {
                        if robot_positions.contains(&(x, y)) {
                            print!("0");
                        } else {
                            print!(".");
                        }
                    }
                }
                println!("\n"); //start a new line //PUT BREAKPOINT HERE TO SOLVE PART 2
            }
        }
    }
}

///Returns the safety factor after 100 seconds have elapsed
fn solution_part1(file_path: &str, (room_width, room_height): (usize, usize)) -> usize {
    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/.
    //Note the position is never negative.
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let raw_input = fs::read_to_string(file_path).expect("failed to open file");

    let robots: Vec<Robot> = re
        .captures_iter(&raw_input)
        .map(|c| {
            let (_, [p_x, p_y, v_x, v_y]) = c.extract();
            let position: (usize, usize) =
                (p_x.parse::<usize>().unwrap(), p_y.parse::<usize>().unwrap());
            let velocity: (i32, i32) = (v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap());
            Robot::new(position, velocity)
        })
        .collect();

    let mut room = security::Room::new(robots, room_width, room_height);

    room.elapse_time(100);

    room.safety_factor()
}

///Find the fewest number of seconds that must elapse for the robots to arrange themselves into a tree.
/// We do this by inspecting printed output.
pub fn solution_part2(file_path: &str, (room_width, room_height): (usize, usize)) {
    //We use a Regex, the following website was helpful https://rustexp.lpil.uk/.
    //Note the position is never negative.
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    let raw_input = fs::read_to_string(file_path).expect("failed to open file");

    let robots: Vec<Robot> = re
        .captures_iter(&raw_input)
        .map(|c| {
            let (_, [p_x, p_y, v_x, v_y]) = c.extract();
            let position: (usize, usize) =
                (p_x.parse::<usize>().unwrap(), p_y.parse::<usize>().unwrap());
            let velocity: (i32, i32) = (v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap());
            Robot::new(position, velocity)
        })
        .collect();

    let mut room = security::Room::new(robots, room_width, room_height);

    let mut count = 0;
    room.check_tree(count);

    loop {
        room.elapse_time(1);
        count += 1;
        room.check_tree(count);
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        //The room is 101 tiles wide and 103 tiles tall
        dbg!(solution_part1("puzzle_inputs/day14.txt", (101, 103)));

        //Put breakpoint and run debug from main to solve part 2
        //dbg!(solution_part2("puzzle_inputs/day14.txt", (101, 103)));
    }

    #[test]
    fn example_part1() {
        //The example, the robots are in a space which is only 11 tiles wide and 7 tiles tall
        let result = solution_part1("puzzle_inputs/day14example.txt", (11, 7));
        assert_eq!(result, 12);
    }
}
