use std::collections::HashMap;
use std::fs;

//The idea for part 1: build a Hash map where key is page_number and value is pages that must come before page_number.
//Parse each update as a vector.
//Iterate on the update vector and for each entry check it is not in vector must_come_before
//and if check successful then append table(entry) to must_come_before

pub struct Update {
    data: Vec<usize>,
}

impl Update {
    pub fn new(data: Vec<usize>) -> Update {
        Update { data }
    }

    ///Finds out if this update is correct or not according to some list of rules (given as a Hash Table)
    pub fn is_correct(&self, rule_set: &HashMap<usize, Vec<usize>>) -> bool {
        //a vector of pages we must not encounter.
        //if X|Y then once we encounter Y we must not encouter X later in the update
        let mut must_not_see = vec![];

        for page in &self.data {
            if must_not_see.contains(page) {
                return false;
            } else if let Some(extension_value) = rule_set.get(page) {
                must_not_see.extend(extension_value);
            }
        }

        //if we got here that means this update is correct
        true
    }

    ///Corrects the update according to the rule set
    pub fn correct_update(&mut self, rule_set: &HashMap<usize, Vec<usize>>) {
        'outer: loop {
            //the key is some page number X, the value is the first index in self.data for a page number Y
            //such that X|Y
            let mut offending_index: HashMap<_, _> = HashMap::new();

            for index in 0..self.data.len() {
                let page = self.data[index];
                if let Some(first_index) = offending_index.get(&page) {
                    //this update violates X|Y. That is this page X comes after some page Y.
                    //We attempt to fix this update by inserting X *before* the index of the first such Y
                    //since a correct ordering is *guaranteed* to exist, we know this terminates
                    //(after swapping i and j, we know sorting i-j w.r.t to the rule_set terminates
                    //[you can see why by induction] and
                    //then the first place problems can occur is at index j+1- so slowly this entire process terminates)

                    //Note first_index < index
                    self.data.remove(index); // Note self.data[index] = page
                    self.data.insert(*first_index, page);
                    continue 'outer;
                } else if let Some(extension_values) = rule_set.get(&page) {
                    for entry in extension_values {
                        offending_index.entry(entry).or_insert(index);
                    }
                }
            }

            //if we got here that means this update is correct
            return;
        }
    }
}

///Returns the sum of the middle page numbers of correct updates (doesn't correct incorrect updates)
fn solution_part1(file_path: &str) -> usize {
    let mut rule_set: HashMap<_, _> = HashMap::new();

    let mut correct_mid_page_sum = 0;

    for line in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
    {
        if let Some((raw_x, raw_y)) = line.split_once('|') {
            //means the line is X|Y so we want to add X to the list of pages that must come before Y
            let (value, key): (usize, usize) = (raw_x.parse().unwrap(), raw_y.parse().unwrap());
            rule_set
                .entry(key)
                .and_modify(|vec: &mut Vec<usize>| vec.push(value))
                .or_insert(vec![value]);
        } else if !line.is_empty() {
            //means the line is an update (a comma seperated list of numbers)
            let update = line
                .split(',')
                .map(|raw| raw.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let update = Update::new(update);
            if update.is_correct(&rule_set) {
                //we are guaranteed updates have an odd length and in rust integer divisions are rounded down automatically
                let mid_index = update.data.len() / 2;
                correct_mid_page_sum += update.data[mid_index];
            }
        }
    }

    correct_mid_page_sum
}

///Returns the sum of the middle page numbers of *just* incorrect updates after correcting them
fn solution_part2(file_path: &str) -> usize {
    let mut rule_set: HashMap<_, _> = HashMap::new();

    let mut mid_page_sum = 0;

    for line in fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
    {
        if let Some((raw_x, raw_y)) = line.split_once('|') {
            //means the line is X|Y so we want to add X to the list of pages that must come before Y
            let (value, key): (usize, usize) = (raw_x.parse().unwrap(), raw_y.parse().unwrap());
            rule_set
                .entry(key)
                .and_modify(|vec: &mut Vec<usize>| vec.push(value))
                .or_insert(vec![value]);
        } else if !line.is_empty() {
            //means the line is an update (a comma seperated list of numbers)
            let update = line
                .split(',')
                .map(|raw| raw.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut update = Update::new(update);
            if !update.is_correct(&rule_set) {
                //correct the update
                update.correct_update(&rule_set);
                //we are guaranteed updates have an odd length and in rust integer divisions are rounded down automatically
                let mid_index = update.data.len() / 2;
                mid_page_sum += update.data[mid_index];
            }
        }
    }

    mid_page_sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day5.txt"));
        dbg!(solution_part2("puzzle_inputs/day5.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day5example.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day5example.txt");
        assert_eq!(result, 123);
    }
}
