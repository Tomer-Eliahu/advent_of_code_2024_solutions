use std::fs;

///A Matrix holds a vector of vectors of chars
pub struct Matrix {
    data: Vec<Vec<char>>,
    row_num: usize,
    col_num: usize,
}

impl Matrix {
    //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
    pub fn new(data: Vec<Vec<char>>) -> Matrix {
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

    ///Find number of times XMAS appears where the X appears in current row and col index (looks in all directions)
    pub fn find_xmas(&self, row_index: usize, col_index: usize) -> usize {
        let mut potential_match: Vec<[char; 3]> = Vec::with_capacity(8);

        //we use slight optimizations here (no point looking right when doing so goes out of the data bounds)
        let should_look_right = col_index + 3 < self.col_num;
        let should_look_down = row_index + 3 < self.row_num;
        let should_look_left = col_index >= 3; //added 3 to both sides to avoid casting col_index to i32
        let should_look_up = row_index >= 3;

        //Another slight optimization we do is only check for MAS since we already know there is an X
        //at row_index, col_index

        if should_look_right {
            //look right
            potential_match.push(
                self.data[row_index][col_index + 1..=col_index + 3]
                    .try_into()
                    .expect("failed conversion"),
            );

            if should_look_up {
                //look up-right diagonal
                let value = [
                    self.data[row_index - 1][col_index + 1],
                    self.data[row_index - 2][col_index + 2],
                    self.data[row_index - 3][col_index + 3],
                ];
                potential_match.push(value);
            }

            if should_look_down {
                //look down-right diagonal
                let value = [
                    self.data[row_index + 1][col_index + 1],
                    self.data[row_index + 2][col_index + 2],
                    self.data[row_index + 3][col_index + 3],
                ];
                potential_match.push(value);
            }
        }

        if should_look_left {
            //look left
            //silcing here we give us potentially "SAM" when we really want it to be "MAS"
            let value = [
                self.data[row_index][col_index - 1],
                self.data[row_index][col_index - 2],
                self.data[row_index][col_index - 3],
            ];
            potential_match.push(value);

            if should_look_up {
                //look up-left diagonal
                let value = [
                    self.data[row_index - 1][col_index - 1],
                    self.data[row_index - 2][col_index - 2],
                    self.data[row_index - 3][col_index - 3],
                ];
                potential_match.push(value);
            }
            if should_look_down {
                //look down-left diagonal
                let value = [
                    self.data[row_index + 1][col_index - 1],
                    self.data[row_index + 2][col_index - 2],
                    self.data[row_index + 3][col_index - 3],
                ];
                potential_match.push(value);
            }
        }

        if should_look_up {
            //look up
            let value = [
                self.data[row_index - 1][col_index],
                self.data[row_index - 2][col_index],
                self.data[row_index - 3][col_index],
            ];
            potential_match.push(value);
        }
        if should_look_down {
            //look down
            let value = [
                self.data[row_index + 1][col_index],
                self.data[row_index + 2][col_index],
                self.data[row_index + 3][col_index],
            ];
            potential_match.push(value);
        }

        //count the matches
        let mut counter = 0;
        for entry in potential_match {
            if entry == ['M', 'A', 'S'] {
                counter += 1;
            }
        }

        counter
    }

    ///Find if there is an X shaped MAS centered at this point, returning 1 if there is and 0 otherwise
    pub fn find_x_shaped_mas(&self, row_index: usize, col_index: usize) -> usize {
        //we use slight optimizations here (no point looking right when doing so goes out of the data bounds)
        let should_look_right = col_index + 1 < self.col_num;
        let should_look_down = row_index + 1 < self.row_num;
        let should_look_left = col_index >= 1;
        let should_look_up = row_index >= 1;

        //Another slight optimization we do is only check for M and S since we already know there is an A
        //at row_index, col_index

        //We must be able to to look in all directions if there is an X shaped M A S at this point
        if should_look_right && should_look_down && should_look_left && should_look_up {
            let top_left = self.data[row_index - 1][col_index - 1];
            let bot_right = self.data[row_index + 1][col_index + 1];
            if (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M') {
                //the left diagonal is good, we now check the right diagonal

                let top_right = self.data[row_index - 1][col_index + 1];
                let bot_left = self.data[row_index + 1][col_index - 1];
                if (top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M') {
                    //both diagonals good, found an X shaped MAS here
                    return 1;
                }
            }
        }

        //if we got here then there is no X-shaped MAS here
        0
    }
}

///Reads the input text and returns the number of occurences of XMAS (Note we are guaranteed the input is rectanguler)
fn solution_part1(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let data_matrix = Matrix::new(data);

    //We now scan the data_matrix and whenever we find an X we check all possible directions for the rest of the letters

    let mut xmas_counter: usize = 0;

    for (row_index, current_row) in data_matrix.data.iter().enumerate() {
        for (col_index, current_char) in current_row.iter().enumerate() {
            if *current_char == 'X' {
                //Call function to check all directions and return number of matches
                xmas_counter += data_matrix.find_xmas(row_index, col_index);
            }
        }
    }

    xmas_counter
}

///Reads the input text and returns the number of occurences of X *Shaped* MAS
///(Note we are guaranteed the input is rectanguler)
fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let data_matrix = Matrix::new(data);

    //We now scan the data_matrix and whenever we find an A we check all possible directions for the rest of the letters

    let mut x_counter: usize = 0;

    for (row_index, current_row) in data_matrix.data.iter().enumerate() {
        for (col_index, current_char) in current_row.iter().enumerate() {
            if *current_char == 'A' {
                //Call function to check all directions and return number of matches
                x_counter += data_matrix.find_x_shaped_mas(row_index, col_index);
            }
        }
    }

    x_counter
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day4.txt"));
        dbg!(solution_part2("puzzle_inputs/day4.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day4example_part1.txt");
        assert_eq!(result, 18);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day4example_part2.txt");
        assert_eq!(result, 9);
    }
}
