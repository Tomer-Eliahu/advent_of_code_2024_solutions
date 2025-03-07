use std::fs;

use graph::Graph;

//part 1 solution notes: Look at the graph of all connections. We want to find all possible subgraphs
//that form a complete subgraph with 3 vertices (all sets of 3 computers such that each computer is connected
//to the other 2 computers in the set).

//If we can consturct a HashMap where the key is the name of the vertex and
//the value is a list of all vertices connected to that vertex (use a HashSet for the list).

//Then for finding a such a subgraph with vertex 'ta' we look at all the connections to 'ta'.
//Say the first connection is 'kh'. The list of vertices that form such a subgraph (with vertices 'ta' and 'kh') is
//the set interesection of the list of connections to 'ta' with the list of connections to 'kh'.

pub mod graph {
    use std::{
        collections::{HashMap, HashSet},
        mem,
    };

    use itertools::Itertools;

    //See explanation why we implement Graph this way at the end of this mod.
    pub struct Graph {
        //Note the lifetime of the string slices are static as they are slices of a String's heap data
        //Note that this is *not* actually a self-referential struct (so no pinning needed).
        network: &'static str,
        connections: HashMap<&'static str, HashSet<&'static str>>,

        //We need to maintain a raw mut pointer to the data of network (what network points at)
        //to have a pointer with the correct provenance for the Drop impl.
        //Note we only use it in the Drop implementation for Graph.
        _raw_network: *mut str,
    }

    impl Graph {
        pub fn build(network: String) -> Graph {
            let slice: Box<str> = network.into_boxed_str();

            //Note: We use Box::leak to get a &'static str.
            //We also make sure to avoid leaking memory later on by impl Drop for Graph.
            //(just dropping the returned reference from Box::leak will cause a memory leak).
            let static_network: &'static mut str = Box::<str>::leak(slice);
            let _raw_network: *mut str = &raw mut *static_network;
            //Note we turn &mut str (what Box::leak returns) into &str (via reborrowing).
            //Note this puts a new immutable reference at the top of the stack in the Stacked Borrows
            //memory model.
            let static_network: &'static str = static_network;

            let mut connections = HashMap::new();

            for raw_connection in static_network.lines() {
                let Some((a, b)) = raw_connection.split_once("-") else {
                    panic!("connection should be of the form foo-bar")
                };

                //insert b into the list of connections for a
                //and a into the list of connections of b
                connections
                    .entry(a)
                    .and_modify(|e: &mut HashSet<&str>| {
                        e.insert(b);
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(b);
                        set
                    });

                connections
                    .entry(b)
                    .and_modify(|e: &mut HashSet<&str>| {
                        e.insert(a);
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(a);
                        set
                    });
            }

            Graph {
                network: static_network,
                connections,
                _raw_network,
            }
        }

        ///Find all possible subgraphs that form a complete subgraph with 3 vertices
        /// where at least one vertex starts with the letter 't'.
        ///
        /// That is: find all sets of 3 computers such that each computer is connected
        ///to the other 2 computers in the set and at least one computer has a name that starts with 't'.
        pub fn find_subgraphs(&'_ self) -> HashSet<[&'_ str; 3]> {
            let mut subgraphs: HashSet<[&str; 3]> = HashSet::new();

            for (&vertex_a, a_connect) in self
                .connections
                .iter()
                .filter(|(vertex_a, _)| vertex_a.starts_with("t"))
            {
                for &vertex_b in a_connect {
                    let b_connect = self.connections.get(vertex_b)
                    .expect("vertex b is connected to vertex a, so it should also be a key in self.connections");

                    //The set intersection of a_connect and b_connect gives us all possible
                    //vertex_c options such that verticies a, b, c, form a complete subgraph.

                    for &vertex_c in a_connect.intersection(b_connect) {
                        //We have to sort the set to avoid double counting subgraphs in the HashSet
                        //(we consider [a,b,c] to be the same as [c,a,b] )
                        let mut set = [vertex_a, vertex_b, vertex_c];
                        set.sort_unstable();

                        subgraphs.insert(set);
                    }
                }
            }

            subgraphs
        }

        ///Returns the password to the LAN party: returns the members of the largest complete subgraph of the network
        /// sorted alphabetically.
        ///
        /// **Note:** We know for this specific puzzle that the largest complete subgraph will have a computer
        /// that starts with the letter 't'. We can also solve this problem in general (without using this fact),
        /// by commenting out the .filter line below.
        pub fn find_largest_subgraph(&self) -> String {
            let mut candidates: Vec<HashSet<&'static str>> = Vec::new();

            for (&vertex, vertex_connect) in self
                .connections
                .iter()
                .filter(|(vertex, _)| vertex.starts_with("t"))
            {
                //**Note:** We know for this specific puzzle that the largest complete subgraph will have a computer
                // that starts with the letter 't'.

                //potential is a complete subgraph candidate
                let mut potential: HashSet<&'static str> = vertex_connect.clone();
                potential.insert(vertex);

                candidates.push(potential);
            }

            #[cfg(test)]
            {
                println!("There are {} candidates", candidates.len());
            };

            let largest = self.get_largest(candidates);

            largest.into_iter().sorted().join(",")
        }

        ///Takes a a list of candidates that may contain within themselves the largest complete subgraph of this Graph.
        /// Returns a HashSet of the elements in the largest complete subgraph
        fn get_largest(&self, candidates: Vec<HashSet<&'static str>>) -> HashSet<&'static str> {
            //We know from part 1 that there are subgraphs of size 3 so the largest subgraph among
            //all the candidates must have size >= 3.
            let mut largest_found = 2;

            candidates
                .into_iter()
                .fold(HashSet::new(), |mut acc, candidate| {
                    if let Some(potential) =
                        self.get_largest_within_candidate(candidate, &mut largest_found)
                    {
                        //Note if Some is returned, we know potential.len() > acc.len()
                        acc = potential;
                    }
                    acc
                })
        }

        ///Finds and returns (one of) the largest complete subgraph within a candidate.
        /// Note that if the size of the largest complete subgraph is <= size, then None is returned.
        fn get_largest_within_candidate(
            &self,
            candidate: HashSet<&'static str>,
            size: &mut usize,
        ) -> Option<HashSet<&'static str>> {
            //We do this so we only sort the candidate once
            let candidate = candidate.into_iter().sorted().collect_vec();

            //We have already found a subgraph that is 'size' big among the candidates, so it is only worth
            //looking for potentially larger subgraphs
            let min = *size + 1;

            for num_vertices in (min..=candidate.len()).rev() {
                //Iterate over all possible combinations of num_vertices elements of candidate
                let combinations = candidate.iter().copied().combinations(num_vertices);

                //Note this intialization value for bad index is fine as each combination is num_vertices long
                let (mut bad_idx, mut bad_comb) = (num_vertices + 10, Vec::new());

                'comb: for comb in combinations {
                    //We take advantage of combinations being sorted
                    //to avoid recalculating long intersection chains repeatdedly.
                    if bad_idx != num_vertices + 10 && comb.starts_with(&bad_comb[..=bad_idx]) {
                        continue 'comb;
                    }

                    //This is a num_vertices length vector (so it is not empty)
                    let mut intersect = self.connections.get(comb[0]).unwrap().clone();

                    //We also need to insert this vertex into the intersection
                    intersect.insert(comb[0]);

                    for (index, &vertex) in comb.iter().enumerate().skip(1) {
                        let mut other = self.connections.get(vertex).unwrap().clone();
                        other.insert(vertex);

                        intersect = intersect.intersection(&other).copied().collect();

                        if intersect.len() < num_vertices {
                            //update bad_index and bad_value: there is no use looking
                            //at combinations where this combination is a prefix of those combinations
                            (bad_idx, bad_comb) = (index, comb);
                            continue 'comb;
                        }
                    }

                    //If the interesection has length num_vertices, then we have found a complete subgraph.
                    if intersect.len() == num_vertices {
                        //Note num_vertices >= min > size. So we update the size of the largest complete subgraph
                        //found among the candidates
                        *size = num_vertices;
                        return Some(intersect);
                    }
                }
            }

            None
        }
    }

    impl Drop for Graph {
        fn drop(&mut self) {
            //To avoid leaking memory (as a result of using Box::leak in Graph::build)
            //we must do the following.

            //drop the old connections
            mem::drop(mem::take(&mut self.connections));

            //drop the network
            {
                mem::take(&mut self.network);
            };

            //SAFETY: We are reconstructing a valid Box from self._raw_network.
            //Note that self._raw_network was obtained directly from Box::leak.
            //Also note there is not anything else that points where _raw_network points.
            unsafe {
                mem::drop(Box::<str>::from_raw(self._raw_network));
            };
        }
    }

    //Alternative way to implement Graph (see explanation why this is bad below):

    // struct GraphInner<'a> {
    //     network_slice: &'a str,
    //     connections: HashMap<&'a str, HashSet<&'a str>>,
    // }

    // impl <'a> GraphInner<'a> {
    //     pub fn build(network: &'a str) -> GraphInner<'a>  {
    //      ...
    //     }
    // }

    //Note this way means the Graph created cannot outlive the String (or &str)
    //from which it was made. This would mean we could not construct a Graph in one function
    //and then return it by value to another function!

    //We can observe this:

    // fn fails() -> (String, &String) {
    //     let a = String::from("Hello");

    //     let mut thing = (a, &String::from("init"));
    //     thing.1 = &thing.0;

    //     thing
    // }

    // fn also_fails() ->(String, &String) {
    //     let a = String::from("Hello");
    //     (a, &a)
    // }
}

///Returns the number of sets of three inter-connected computers where
/// at least one computer has a name that starts with t
fn solution_part1(file_path: &str) -> usize {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let graph = Graph::build(data);

    graph.find_subgraphs().len()
}

///Returns the password to the LAN party: returns the members of the largest complete subgraph of the network
/// sorted alphabetically.
fn solution_part2(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let graph = Graph::build(data);

    graph.find_largest_subgraph()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day23.txt"));
        dbg!(solution_part2("puzzle_inputs/day23.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day23example.txt");
        assert_eq!(result, 7);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day23example.txt");
        assert_eq!(result, "co,de,ka,ta");
    }
}
