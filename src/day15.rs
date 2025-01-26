use std::fs;

use robot::{Regular, RobotPatrol, Wide};

//Part 1 solution notes:
//Have a function that applies a move to the robot for every char of the input seqeunce we read.
//Note that if checking if a move up is valid from the robots position, what we really want to know is
//if there is a non-box (O), non-wall (#) space when looking up from the robot's position.
//That is we want to know if there is a free-space (.) when looking up from the robot's position.

//We see that it is enough to just keep track of 3 potential moves (i.e. put a free space
//where the robot is, move the robot 1 space, and update the free space we find).

pub mod robot {
    use std::{collections::HashSet, fmt::Debug, marker::PhantomData};

    //Note that these are zero-sized types (a zero-cost abstraction)
    pub struct Wide;
    pub struct Regular;

    pub trait MapType {}

    impl MapType for Wide {}

    impl MapType for Regular {}

    //For debugging- I made Matrix pub and data pub
    #[derive(Clone)]
    pub struct Matrix<T> {
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

    impl Debug for Matrix<char> {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in self.data.iter() {
                writeln!(_f)?;
                for char in row.iter() {
                    write!(_f, "{}", char)?;
                }
            }

            Ok(())
        }
    }
    //For debugging- I made map pub
    pub struct RobotPatrol<T: MapType> {
        robot_location: (usize, usize),
        pub map: Matrix<char>,
        _marker: PhantomData<T>,
    }

    impl<T: MapType> RobotPatrol<T> {
        ///Find the location of the guard in a map.
        ///That is find indices (i,j) such that at (i,j) == '@'
        fn find_robot(map: &Matrix<char>) -> Result<(usize, usize), &'static str> {
            for (row_index, row) in map.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    if *char == '@' {
                        return Ok((row_index, col_index));
                    }
                }
            }

            Err("Could not find robot")
        }

        pub fn new(map: Vec<Vec<char>>) -> RobotPatrol<T> {
            let map = Matrix::new(map);

            let robot_location = RobotPatrol::<T>::find_robot(&map).unwrap();

            Self {
                robot_location,
                map,
                _marker: PhantomData,
            }
        }
    }

    impl RobotPatrol<Regular> {
        ///Returns the sum of all boxes' GPS coordinates.
        /// The GPS coordinate of a box is equal
        /// to 100 times its distance from the top edge of the map plus its distance from the left edge of the map.
        /// Note this is simply 100 * row_index + col_index.
        pub fn sum_gps(&self) -> usize {
            let mut sum = 0;

            for (row_index, row) in self.map.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    if *char == 'O' {
                        sum += 100 * row_index + col_index;
                    }
                }
            }

            sum
        }

        pub fn move_up(&mut self) {
            let (mut row, col) = (self.robot_location.0, self.robot_location.1);

            loop {
                //Note we know the map is fenced on all sides by # so we don't have to check that row-1>=0
                //because it is guaranteed.
                row -= 1;

                match self.map.data[row][col] {
                    '.' => {
                        //There are 2 cases, either we hit a free space immediately after the robot,
                        //Or we hit a free space after some sequence of boxes
                        if self.map.data[row + 1][col] == '@' {
                            self.map.data[row][col] = '@';
                            self.map.data[row + 1][col] = '.';
                        } else {
                            //In this case we need to update the map 3 times
                            //(all other updates are replacing a box with another box)
                            self.map.data[row][col] = 'O';

                            self.map.data[self.robot_location.0][self.robot_location.1] = '.';
                            self.map.data[self.robot_location.0 - 1][self.robot_location.1] = '@';
                        }

                        //Since the robot moved we must also update the robot location
                        self.robot_location = (self.robot_location.0 - 1, self.robot_location.1);

                        break;
                    }
                    '#' => break,
                    'O' => continue,
                    _ => unreachable!(),
                }
            }
        }

        pub fn move_down(&mut self) {
            let (mut row, col) = (self.robot_location.0, self.robot_location.1);

            loop {
                row += 1;

                match self.map.data[row][col] {
                    '.' => {
                        //There are 2 cases, either we hit a free space immediately after the robot,
                        //Or we hit a free space after some sequence of boxes
                        if self.map.data[row - 1][col] == '@' {
                            self.map.data[row][col] = '@';
                            self.map.data[row - 1][col] = '.';
                        } else {
                            //In this case we need to update the map 3 times
                            //(all other updates are replacing a box with another box)
                            self.map.data[row][col] = 'O';

                            self.map.data[self.robot_location.0][self.robot_location.1] = '.';
                            self.map.data[self.robot_location.0 + 1][self.robot_location.1] = '@';
                        }

                        //Since the robot moved we must also update the robot location
                        self.robot_location = (self.robot_location.0 + 1, self.robot_location.1);

                        break;
                    }
                    '#' => break,
                    'O' => continue,
                    _ => unreachable!(),
                }
            }
        }

        pub fn move_right(&mut self) {
            let (row, mut col) = (self.robot_location.0, self.robot_location.1);

            loop {
                col += 1;

                match self.map.data[row][col] {
                    '.' => {
                        //There are 2 cases, either we hit a free space immediately after the robot,
                        //Or we hit a free space after some sequence of boxes
                        if self.map.data[row][col - 1] == '@' {
                            self.map.data[row][col] = '@';
                            self.map.data[row][col - 1] = '.';
                        } else {
                            //In this case we need to update the map 3 times
                            //(all other updates are replacing a box with another box)
                            self.map.data[row][col] = 'O';

                            self.map.data[self.robot_location.0][self.robot_location.1] = '.';
                            self.map.data[self.robot_location.0][self.robot_location.1 + 1] = '@';
                        }

                        //Since the robot moved we must also update the robot location
                        self.robot_location = (self.robot_location.0, self.robot_location.1 + 1);

                        break;
                    }
                    '#' => break,
                    'O' => continue,
                    _ => unreachable!(),
                }
            }
        }

        pub fn move_left(&mut self) {
            let (row, mut col) = (self.robot_location.0, self.robot_location.1);

            loop {
                col -= 1;

                match self.map.data[row][col] {
                    '.' => {
                        //There are 2 cases, either we hit a free space immediately after the robot,
                        //Or we hit a free space after some sequence of boxes
                        if self.map.data[row][col + 1] == '@' {
                            self.map.data[row][col] = '@';
                            self.map.data[row][col + 1] = '.';
                        } else {
                            //In this case we need to update the map 3 times
                            //(all other updates are replacing a box with another box)
                            self.map.data[row][col] = 'O';

                            self.map.data[self.robot_location.0][self.robot_location.1] = '.';
                            self.map.data[self.robot_location.0][self.robot_location.1 - 1] = '@';
                        }

                        //Since the robot moved we must also update the robot location
                        self.robot_location = (self.robot_location.0, self.robot_location.1 - 1);

                        break;
                    }
                    '#' => break,
                    'O' => continue,
                    _ => unreachable!(),
                }
            }
        }
    }

    impl RobotPatrol<Wide> {
        ///Returns the sum of all boxes' GPS coordinates.
        /// The GPS coordinate of a larger box is equal
        /// to 100 times its distance from the top edge of the map plus its distance from the left edge of the map.
        /// Note this is simply 100 * row_index + col_index of the '[' part of the box.
        pub fn sum_gps(&self) -> usize {
            let mut sum = 0;

            for (row_index, row) in self.map.data.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    if *char == '[' {
                        sum += 100 * row_index + col_index;
                    }
                }
            }

            sum
        }

        //Note that after we reconsider a column we must add an instruction to put a free space
        //for the inital movement.
        //To see why consider pushing up when in this situation:

        // ##[]..[][]..##......[]##[]....[]..##..[][]....[]....######....##..[]...[]...........[]......[]....##
        // ##..[]..[]......[][]....[]..[]##[]..[][]....[]......[]....##[]........[]......##..................##
        // ##..[]......##..[]....[]..[]..[]....[][]..##..[]..[][]............##.[][].............##......##..##
        // ##....[][][]..[]....[]....[][][][]..[]..##..........[]##......##[].....@............[]....[]....####
        // ##....[]..........[]##....[][][]..[]..............##....##..[]..##....[][].........[].............##
        // ######[]##..............##..............##[]##..##[][]..[]......[].........[].....[]..[]..........##

        //When we reconsider a column it is as if we rediscover it.
        pub fn move_up(&mut self) {
            let mut planned_moves: Vec<(usize, usize, char)> = vec![];

            let (mut row, col) = (self.robot_location.0, self.robot_location.1);

            //Because moving up might cause a box to move both of its parts up,
            //we introduce in move_up and move_down the concept of the column_range;
            let mut col_range: (usize, usize) = (col, col);

            planned_moves.push((row, col, '.'));

            let mut skip_cols: HashSet<usize> = HashSet::new();
            let mut cols_to_reconsider: Vec<usize> = vec![];

            let valid_move = 'outer: loop {
                let mut all_free = true;

                row -= 1;

                for col in col_range.0..=col_range.1 {
                    if skip_cols.contains(&col) {
                        continue;
                    }

                    match self.map.data[row][col] {
                        '.' => {
                            planned_moves.push((row, col, self.map.data[row + 1][col]));

                            //skip this col untill further notice
                            skip_cols.insert(col);
                        }
                        '#' => break 'outer false,
                        '[' => {
                            planned_moves.push((row, col, self.map.data[row + 1][col]));

                            //If the right-most edge of this row is '[',
                            //we need to account for the ']' part of this box
                            if col == col_range.1 {
                                col_range.1 += 1;
                                planned_moves.push((row, col + 1, '.'));
                            } else if skip_cols.contains(&(col + 1)) {
                                //We are effectively rediscovering this column!

                                //we know at col+1 there is a ']', and we therefore must reconsider col+1 once the iteration
                                //over the current range is over.
                                planned_moves.push((row, col + 1, '.'));
                                cols_to_reconsider.push(col + 1);
                            }

                            all_free = false;
                            continue;
                        }
                        ']' => {
                            planned_moves.push((row, col, self.map.data[row + 1][col]));
                            //If the left-most edge of this row is ']',
                            //we need to account for the '[' part of this box
                            if col == col_range.0 {
                                col_range.0 -= 1;
                                planned_moves.push((row, col - 1, '.'));
                            } else if skip_cols.contains(&(col - 1)) {
                                //we know at col-1 there is a '[', and we therefore must reconsider col-1 once the iteration
                                //over the current range is over.
                                planned_moves.push((row, col - 1, '.'));
                                cols_to_reconsider.push(col - 1);
                            }

                            all_free = false;
                            continue;
                        }
                        _ => unreachable!(),
                    }
                }

                while let Some(col) = cols_to_reconsider.pop() {
                    skip_cols.remove(&col);
                }

                if all_free {
                    //if we found enough free spaces to move everything needed up
                    break true;
                }
            };

            if valid_move {
                while let Some((row, col, char)) = planned_moves.pop() {
                    self.map.data[row][col] = char;
                }

                //Since the robot moved we must also update the robot location
                self.robot_location = (self.robot_location.0 - 1, self.robot_location.1);
            }
        }

        pub fn move_down(&mut self) {
            let mut planned_moves: Vec<(usize, usize, char)> = vec![];

            let (mut row, col) = (self.robot_location.0, self.robot_location.1);

            let mut col_range: (usize, usize) = (col, col);

            planned_moves.push((row, col, '.'));

            let mut skip_cols: HashSet<usize> = HashSet::new();
            let mut cols_to_reconsider: Vec<usize> = vec![];

            let valid_move = 'outer: loop {
                let mut all_free = true;

                row += 1;

                for col in col_range.0..=col_range.1 {
                    if skip_cols.contains(&col) {
                        continue;
                    }

                    match self.map.data[row][col] {
                        '.' => {
                            planned_moves.push((row, col, self.map.data[row - 1][col]));
                            //We also must skip this column in future ranges
                            //since a free space doesn't get pushed up/down
                            skip_cols.insert(col);
                        }
                        '#' => break 'outer false,
                        '[' => {
                            planned_moves.push((row, col, self.map.data[row - 1][col]));

                            //If the right-most edge of this row is '[',
                            //we need to account for the ']' part of this box
                            if col == col_range.1 {
                                col_range.1 += 1;
                                planned_moves.push((row, col + 1, '.'));
                            } else if skip_cols.contains(&(col + 1)) {
                                //we know at col+1 there is a ']', and we therefore must reconsider col+1 once the iteration
                                //over the current range is over.
                                planned_moves.push((row, col + 1, '.'));
                                cols_to_reconsider.push(col + 1);
                            }

                            all_free = false;
                            continue;
                        }
                        ']' => {
                            planned_moves.push((row, col, self.map.data[row - 1][col]));

                            //If the left-most edge of this row is ']',
                            //we need to account for the '[' part of this box
                            if col == col_range.0 {
                                col_range.0 -= 1;
                                planned_moves.push((row, col - 1, '.'));
                            } else if skip_cols.contains(&(col - 1)) {
                                //we know at col-1 there is a '[', and we therefore must reconsider col-1 once the iteration
                                //over the current range is over.
                                planned_moves.push((row, col - 1, '.'));
                                cols_to_reconsider.push(col - 1);
                            }

                            all_free = false;
                            continue;
                        }
                        _ => unreachable!(),
                    }
                }

                while let Some(col) = cols_to_reconsider.pop() {
                    skip_cols.remove(&col);
                }

                if all_free {
                    //if we found enough free spaces to move everything needed up
                    break true;
                }
            };

            if valid_move {
                while let Some((row, col, char)) = planned_moves.pop() {
                    self.map.data[row][col] = char;
                }

                //Since the robot moved we must also update the robot location
                self.robot_location = (self.robot_location.0 + 1, self.robot_location.1);
            }
        }

        //We could combine move_right and move_left into a single function (by using a function pointer
        //to do the needed operation of addition or subtraction).
        //It does potentially hurt performance a bit.
        pub fn move_right(&mut self) {
            let mut planned_moves: Vec<(usize, usize, char)> = vec![];

            let (row, mut col) = (self.robot_location.0, self.robot_location.1);

            planned_moves.push((row, col, '.'));

            let mut last_char = '@';

            let valid_move = loop {
                col += 1;

                match self.map.data[row][col] {
                    '.' => {
                        planned_moves.push((row, col, last_char));
                        break true;
                    }
                    '#' => break false,
                    '[' => {
                        planned_moves.push((row, col, last_char));
                        last_char = '[';
                        continue;
                    }
                    ']' => {
                        planned_moves.push((row, col, last_char));
                        last_char = ']';
                        continue;
                    }
                    _ => unreachable!(),
                }
            };

            if valid_move {
                for (row, col, char) in planned_moves {
                    self.map.data[row][col] = char;
                }

                //Since the robot moved we must also update the robot location
                self.robot_location = (self.robot_location.0, self.robot_location.1 + 1);
            }
        }

        pub fn move_left(&mut self) {
            let mut planned_moves: Vec<(usize, usize, char)> = vec![];

            let (row, mut col) = (self.robot_location.0, self.robot_location.1);

            planned_moves.push((row, col, '.'));

            let mut last_char = '@';

            let valid_move = loop {
                col -= 1;

                match self.map.data[row][col] {
                    '.' => {
                        planned_moves.push((row, col, last_char));
                        break true;
                    }
                    '#' => break false,
                    '[' => {
                        planned_moves.push((row, col, last_char));
                        last_char = '[';
                        continue;
                    }
                    ']' => {
                        planned_moves.push((row, col, last_char));
                        last_char = ']';
                        continue;
                    }
                    _ => unreachable!(),
                }
            };

            if valid_move {
                for (row, col, char) in planned_moves {
                    self.map.data[row][col] = char;
                }

                //Since the robot moved we must also update the robot location
                self.robot_location = (self.robot_location.0, self.robot_location.1 - 1);
            }
        }
    }
}

///Returns the sum of all boxes' GPS coordinates after the robot finishes moving.
/// The GPS coordinate of a box is equal
/// to 100 times its distance from the top edge of the map plus its distance from the left edge of the map.
fn solution_part1(file_path: &str) -> usize {
    let raw_data = fs::read_to_string(file_path).expect("failed to open file");

    let mut lines = raw_data.lines();

    //Note this take_while will eliminate the empty line out of the lines iterator.
    let data = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut patrol: RobotPatrol<Regular> = robot::RobotPatrol::new(data);

    //Now lines strictly contains the move sequence with some newlines (within the move sequence) we should ignore.
    //We can ignore the newlines by simply iterating over lines.
    for line in lines {
        for char in line.chars() {
            match char {
                '^' => patrol.move_up(),
                '>' => patrol.move_right(),
                '<' => patrol.move_left(),
                'v' => patrol.move_down(),
                _ => unreachable!(),
            }
        }
    }

    patrol.sum_gps()
}

///Returns the sum of all boxes' final GPS coordinates of the modified wide map.
fn solution_part2(file_path: &str) -> usize {
    let raw_data = fs::read_to_string(file_path).expect("failed to open file");

    let mut lines = raw_data.lines();

    //Note this take_while will eliminate the empty line out of the lines iterator.
    //Also note we expand each line individually to make the map twice as wide according to the instructions.
    //Note that it is important that replacing @ with @. comes after replacing . with .. .
    let data = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.replace("O", "[]")
                .replace("#", "##")
                .replace(".", "..")
                .replace("@", "@.")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect();

    let mut patrol: RobotPatrol<Wide> = robot::RobotPatrol::new(data);

    //print!("The INITAL data is: \n");
    // dbg!(&patrol.map);

    //Now lines strictly contains the move sequence with some newlines (within the move sequence) we should ignore.
    //We can ignore the newlines by simply iterating over lines.
    for line in lines {
        for char in line.chars() {
            // let old_map = patrol.map.clone();
            match char {
                '^' => patrol.move_up(),
                '>' => patrol.move_right(),
                '<' => patrol.move_left(),
                'v' => patrol.move_down(),
                _ => unreachable!(),
            }

            // let bad_map =
            // 'outer: {

            //     for (row_index, row) in patrol.map.data.iter().enumerate() {
            //         for (col_index, char) in row.iter().enumerate() {
            //             if (*char == '['  && patrol.map.data[row_index][col_index+1] != ']')
            //             || (*char == ']'  && patrol.map.data[row_index][col_index-1] != '['){
            //                 break 'outer true;
            //             }

            //         }
            //     }
            //     false
            // };

            // // if bad_map{
            // //     dbg!(&old_map);
            // //     dbg!(&patrol.map);
            // // }
        }
    }

    //print!("The Final data is: \n");
    //dbg!(&patrol.map);
    patrol.sum_gps()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day15.txt"));
        dbg!(solution_part2("puzzle_inputs/day15.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day15example.txt");
        assert_eq!(result, 10092);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day15example.txt");
        assert_eq!(result, 9021);
    }
}
