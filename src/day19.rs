use std::fs;

use itertools::Itertools;
use towels::Edges;

//Part 1 solution notes:
//We can think of this as a prefix-match problem. This transforms this problem into a graph problem.

//For example say we have the following towels available to us (unlimited copies of each):
//r, wr, b, g, bwu, rb, gb, br

//We want to determine if we can combine these to make each "design pattern". For example:

//brrrg we can make by taking br, then r twice then a g.

//So we can imagine we are at an empty node with edges r, wr, b, g, bwu, rb, gb, br.
//We want to traverse an edge such that our resulting word is a prefix of brrrg.

//Once we have a prefix that is the whole word, we know that design is possible.
//If we get stuck at node with no options to proceed, we know that design is impossible.
//(We are simulating the finite-state machine a.k.a. the finite-state automaton).

pub mod towels {
    use std::collections::{BTreeMap, HashSet};

    pub struct Edges<'a> {
        edges: Vec<(&'a str, usize)>,
    }

    impl<'a> Edges<'a> {
        ///Takes a list of edges, eliminates edges that can be made up from other edges.
        /// For example, we can eliminate edges like "gr" if we know we have "g" and "r" as edges.
        /// Also sorts the edges by length in increasing order (used to optimize the is_design method).
        pub fn build_no_duplicates(edges: &[&'a str]) -> Edges<'a> {
            //Note it is ok to use len below as we know we are dealing with strictly ASCII.
            //We utilize computing lens for efficency in the is_design function.
            let mut edges: Vec<(&str, usize)> =
                edges.iter().map(|&edge| (edge, edge.len())).collect();

            //sort edges by len in increasing order.
            edges.sort_by(|(_, a), (_, b)| a.cmp(b));

            //eliminate edges we can make up from other edges
            let edges: Vec<(&str, usize)> = edges.into_iter().fold(vec![], |mut accum, curr| {
                //See if the current edge can not be made up from previous edges
                if !is_design_inner(&accum, curr.0) {
                    accum.push(curr);
                }

                accum
            });

            Self { edges }
        }

        //We do things this way because is_design_inner relies on the edges being sorted.
        //By making is_design the only pub way to call is_design_inner we can guarantee Edges::build (any version)
        // has been run.
        pub fn is_design(&self, design: &str) -> bool {
            is_design_inner(&self.edges, design)
        }

        ///Just sorts the list of edges
        pub fn build(edges: &[&'a str]) -> Edges<'a> {
            //Note it is ok to use len below as we know we are dealing with strictly ASCII.
            let mut edges: Vec<(&str, usize)> =
                edges.iter().map(|&edge| (edge, edge.len())).collect();

            //sort edges by len in increasing order.
            edges.sort_by(|(_, a), (_, b)| a.cmp(b));

            Self { edges }
        }

        ///Takes as input a target design pattern.
        /// Returns how many different ways we can make the design out of the edges.
        /// If we can't make the design out of the edges returns 0.
        pub fn count_design(&self, design: &str) -> usize {
            let target_len = design.len();

            //We utlize a BTreeMap to avoid doing duplicate work
            //The key is the prefix_len and the value is how many times we encountered this length.
            let mut explored: BTreeMap<usize, usize> = BTreeMap::new();
            explored.insert(0, 1);

            //Note we relay on the invariant that explored contains no duplicates and will be popped in increasing order.
            while let Some((prefix_len, count)) = explored.pop_first() {
                if prefix_len == target_len {
                    return count;
                }

                //For each prefix, we now try to see if prefix+some_edge forms a prefix of our design

                for &(edge, edge_len) in &self.edges {
                    let new_len = prefix_len + edge_len;
                    if new_len > target_len {
                        break; //We do this as edges is sorted
                    }

                    //We already know we can use our edges to make up &design[..prefix_len] (recall 0..0 is empty).
                    //We now test if using this edge will extend this prefix by edge_len.
                    if edge == &design[prefix_len..new_len] {
                        explored
                            .entry(new_len)
                            .and_modify(|e| *e += count)
                            .or_insert(count);
                    }
                }
            }

            //This design is impossible with these edges
            0
        }
    }

    ///Takes as input a list of edges and a target design pattern.
    /// Returns whether using these edges we can make up the design pattern.
    fn is_design_inner(edges: &Vec<(&str, usize)>, design: &str) -> bool {
        let target_len = design.len();

        //A list of the lengths of prefixes we found of our design.
        //It is actually enough to just keep track of the lengths.
        //We start with 0 because "" is a prefix.
        //Note we use a HashSet to avoid duplicate work.
        let mut prefixes = vec![0_usize];
        let mut explored: HashSet<usize> = HashSet::new();
        explored.insert(0);

        while let Some(prefix_len) = prefixes.pop() {
            //For each prefix, we now try to see if prefix+some_edge forms a prefix of our design

            for &(edge, edge_len) in edges {
                let new_len = prefix_len + edge_len;
                if new_len > target_len {
                    break; //We do this as edges is sorted
                }

                //We already know we can use our edges to make up &design[..prefix_len] (recall 0..0 is empty).
                //We now test if using this edge will extend this prefix by edge_len.
                if edge == &design[prefix_len..new_len] {
                    if new_len == target_len {
                        //Meaning we made up the design using only our edges
                        return true;
                    }
                    //If new_len < target_len
                    if explored.insert(new_len) {
                        prefixes.push(new_len);
                    }
                }
            }
        }

        //We exhausted all possibilites
        false
    }
}

///Returns how many of the designs are possible
fn solution_part1(file_path: &str) -> usize {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let mut data_iter: std::str::Lines<'_> = data.lines();

    let edges = data_iter
        .next()
        .map(|edge_list| edge_list.split(",").map(|edge| edge.trim()).collect_vec())
        .unwrap();
    data_iter.next(); //We know this line is empty

    let edges = Edges::build_no_duplicates(&edges);

    data_iter.filter(|&design| edges.is_design(design)).count()
}

/// Returns the sum of the number of different ways you could make each design
fn solution_part2(file_path: &str) -> usize {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let mut data_iter: std::str::Lines<'_> = data.lines();

    let edges = data_iter
        .next()
        .map(|edge_list| edge_list.split(",").map(|edge| edge.trim()).collect_vec())
        .unwrap();
    data_iter.next(); //We know this line is empty

    let edges = Edges::build(&edges);

    data_iter.map(|design| edges.count_design(design)).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day19.txt"));
        dbg!(solution_part2("puzzle_inputs/day19.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day19example.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day19example.txt");
        assert_eq!(result, 16);
    }
}
