use std::{collections::HashMap, fs};

//Part 1 solution notes:
//use iterators to wrap stones. This will enable us to sometimes return 1 stone from processing a stone,
//and sometimes return 2 stones.
//Keep the stones numbers as chars (makes rule 1 and 2 easy).
//Write a function that does 1 blink that takes and returns an iterator over stones.

pub struct Stone {
    pub data: String,
    pub multiplier: usize,
}

pub fn blink(stone_line: Box<dyn Iterator<Item = Stone>>) -> Box<dyn Iterator<Item = Stone>> {
    //Note: each time we do stone_line.map(|stone| process_stone(stone)).flatten() we get a different new
    //type that implements Iterator<Item=String>. That new type is the old type wrapped in FlatMap<..>
    //and also captures the closure type of each map.
    //Unfortunately, we cannot avoid trait objects here.
    //Actually, I suppose we could write blink_num calls of blink by hand
    //or via a macro: let stone_line = blink(stone_line) repeated blink num times
    //and avoid trait objects that way but that is overkill for this puzzle.

    //Part 2 optimization: we note we can reduce the problem.
    //at each stone_line we now eliminate repeated numbers (as they will result in the same number of stones anyway).
    //We keep track of a multiplier per stone.
    let mut stones = HashMap::new();

    for stone in stone_line.flat_map(process_stone) {
        stones
            .entry(stone.data)
            .and_modify(|e| *e += stone.multiplier)
            .or_insert(stone.multiplier);
    }

    Box::new(stones.into_iter().map(|(key, value)| Stone {
        data: key,
        multiplier: value,
    }))
}

fn process_stone(mut stone: Stone) -> Vec<Stone> {
    if stone.data == "0" {
        stone.data.pop();
        stone.data.push('1');
        vec![stone]
    } else if stone.data.len() % 2 == 0 {
        //Note that stone.data.len gives us a result in bytes, not chars.
        //But this is ok here as each digit 0-9 is 1 byte in the String as Rust Strings are UTF-8 encoded.

        //We remove leading 0's from the right stone
        let mut right_stone_data = stone
            .data
            .split_off(stone.data.len() / 2)
            .trim_start_matches('0')
            .to_string();
        //It might be that the right stone was all 0's
        if right_stone_data.is_empty() {
            right_stone_data.push('0');
        }
        let right_stone = Stone {
            data: right_stone_data,
            multiplier: stone.multiplier,
        };

        vec![stone, right_stone]
    } else {
        stone.data = (stone
            .data
            .parse::<usize>()
            .expect("Stone should be engraved with a number")
            * 2024)
            .to_string();
        vec![stone]
    }
}

///Returns the number of stones after blinking blink_num times
fn solution(file_path: &str, blink_num: usize) -> usize {
    //Note we MUST use collect here in the end so that stone_line owns the strings (i.e.
    //we ensure map is consumed before passing stone_line to blink)
    let stone_line: Vec<Stone> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .split(' ')
        .map(|word| Stone {
            data: word.to_string(),
            multiplier: 1,
        })
        .collect::<Vec<Stone>>();

    let mut stone_line: Box<dyn Iterator<Item = Stone>> = Box::new(stone_line.into_iter());
    //Trait objects are needed as the explicit type of stone_line changes with each call to blink
    //(it is wrapped inside Flatten<Map<..>> and also captures the closure type of each map).
    //We could avoid this with a macro if performance was critical.
    for _ in 0..blink_num {
        stone_line = blink(stone_line);
    }

    //We remember to take the multiplier into account
    stone_line.map(|stone| stone.multiplier).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution("puzzle_inputs/day11.txt", 25));
        dbg!(solution("puzzle_inputs/day11.txt", 75));
    }

    #[test]
    fn example_part1() {
        let result = solution("puzzle_inputs/day11example.txt", 25);
        assert_eq!(result, 55312);
    }
}
