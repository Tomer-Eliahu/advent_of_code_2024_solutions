use std::fs;

use logic_gates::System;

pub mod logic_gates {
    use std::collections::{HashMap, HashSet, VecDeque};

    use itertools::Itertools;
    use regex::Regex;

    #[derive(Debug)]
    enum Gate {
        And,
        Or,
        Xor,
    }

    ///An instruction takes 2 inputs and a gate type (one of AND, OR, XOR) and writes the output value to
    /// the output location
    #[derive(Debug)]
    struct Instruction {
        in_1: &'static str,
        in_2: &'static str,
        gate: Gate,
        out: &'static str,
    }

    struct Circuit<'a> {
        in_1: &'a str,
        in_2: &'a str,
        gate: &'a str,
        out: &'a str,
    }

    pub struct System {
        gate_values: HashMap<&'static str, bool>,
        instructions: VecDeque<Instruction>,
    }

    impl System {
        ///Construct and simulate the system of gates and wires.
        pub fn build_and_execute(system: String) -> System {
            let system: &'static str = system.leak();
            //Note the split_once value depends on whether the file is saved with LF end of line sequence or CRLF
            let (gate_values, instructions) = system.split_once("\n\n")
            .expect("The initial gate values should be seperated by an empty line from the Instructions");

            //We use a Regex, the following website was helpful https://rustexp.lpil.uk/
            //initialize gate_values
            let re = Regex::new(r"(.+): (\d)").unwrap();

            let gate_values: HashMap<&str, bool> = re
                .captures_iter(gate_values)
                .map(|c| {
                    let (_, [gate, raw_input]) = c.extract();

                    let input = match raw_input
                        .parse::<usize>()
                        .expect("raw input should be 0 or 1")
                    {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    };

                    (gate, input)
                })
                .collect();

            //initialize instructions
            let re = Regex::new(r"(.+) (.+) (.+) -> (.+)").unwrap();

            let instructions: VecDeque<Instruction> = re
                .captures_iter(instructions)
                .map(|c| {
                    let (_, [in_1, gate_type, in_2, out]) = c.extract();

                    let gate_type = match gate_type {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        _ => unreachable!(),
                    };

                    Instruction {
                        in_1,
                        in_2,
                        gate: gate_type,
                        out,
                    }
                })
                .collect();

            let mut system = System {
                gate_values,
                instructions,
            };
            system.execute();

            system
        }

        ///Executes the Instructions for the system and updates the status of the logic_gates accordingly
        fn execute(&mut self) {
            while let Some(instruction) = self.instructions.pop_front() {
                match (
                    self.gate_values.get(instruction.in_1),
                    self.gate_values.get(instruction.in_2),
                ) {
                    (Some(&in_1), Some(&in_2)) => {
                        //Execute this instruction
                        let output_value = match instruction.gate {
                            Gate::And => in_1 && in_2,
                            Gate::Or => in_1 || in_2,
                            Gate::Xor => (!in_1 && in_2) || (in_1 && !in_2),
                        };
                        //We know each gate is only ever given 1 value
                        self.gate_values.insert(instruction.out, output_value);
                    }
                    _ => {
                        //We are missing at least one of the inputs information (we have yet to compute it).
                        //Delay executing this instruction
                        self.instructions.push_back(instruction);
                    }
                }
            }
        }

        ///Returns the decimal number this system outputs on the wires starting with z
        pub fn decimal_output(&self) -> usize {
            let binary_string: String = self
                .gate_values
                .iter()
                .filter(|(wire, _)| wire.starts_with("z"))
                .sorted()
                .map(|(_, &value)| match value {
                    true => "1",
                    false => "0",
                })
                .rev()
                .collect();

            usize::from_str_radix(&binary_string, 2).unwrap()
        }
    }

    //Part 2: We are given that this system is trying to do binary addition of 2 numbers (each 45 bits long).

    ///Returns the names of the output wires of the Circuits we needed to swap
    pub fn build_and_evaluate(system: String) -> String {
        //Note the split_once value depends on whether the file is saved with LF end of line sequence or CRLF.
        //We ignore the initial input values
        let (_, instructions) = system.split_once("\n\n").expect(
            "The initial gate values should be seperated by an empty line from the Instructions",
        );

        //initialize instructions
        let re = Regex::new(r"(.+) (.+) (.+) -> (.+)").unwrap();

        //For key out: the value is out in terms of x's and y's
        let mut out_eval: HashMap<&str, String> = HashMap::new();

        let mut out_instruct: HashMap<&str, Circuit> = re
            .captures_iter(instructions)
            .map(|c| {
                let (_, [in_1, gate_type, in_2, out]) = c.extract();

                let insturct = Circuit {
                    in_1,
                    in_2,
                    gate: gate_type,
                    out,
                };

                (out, insturct)
            })
            .collect();

        //Swap the pairs we found needed to be swapped (found this
        //by inspecting the printout from the evaluate function)
        {
            let mut swap_pairs: HashSet<(&str, &str)> = HashSet::new();

            //Found the following swaps:
            swap_pairs.insert(("swt", "z07"));
            swap_pairs.insert(("pqc", "z13"));
            swap_pairs.insert(("rjm", "wsv"));
            swap_pairs.insert(("bgs", "z31"));

            for (swap_a, swap_b) in swap_pairs {
                let mut out_a = out_instruct.remove(swap_a).unwrap();
                let mut out_b = out_instruct.remove(swap_b).unwrap();
                out_a.out = swap_b;
                out_b.out = swap_a;
                out_instruct.insert(swap_a, out_b);
                out_instruct.insert(swap_b, out_a);
            }
        }

        //Fill in out_eval
        let mut vec: VecDeque<_> = out_instruct.iter_mut().collect();

        while let Some((key, value)) = vec.pop_front() {
            if value.in_1.starts_with(['x', 'y']) && value.in_2.starts_with(['x', 'y']) {
                //We make sure all of our building blocks are of the form x CMD y
                if value.in_1.starts_with("y") {
                    (value.in_1, value.in_2) = (value.in_2, value.in_1);
                }

                let out_value = format!("({} {} {})", value.in_1, value.gate, value.in_2);
                out_eval.insert(value.out, out_value);
            } else if let (Some(expr_1), Some(expr_2)) =
                (out_eval.get(value.in_1), out_eval.get(value.in_2))
            {
                let out_value = format!("({expr_1} {} {expr_2})", value.gate);

                out_eval.insert(value.out, out_value);
            } else {
                vec.push_back((key, value));
            }
        }

        evaluate(out_instruct, out_eval)
    }

    ///We should have something like for our System:
    ///
    ///     z00 = x00 XOR y00
    ///     z01 = ((x01 XOR y01) XOR (x00 AND y00))
    ///      
    ///                                            (we don't include the rightmost ')' in A1)
    ///     z02= ((x02 XOR y02) XOR ((x01 AND y01) OR ((xo1 XOR y01) AND (x00 AND y00)))           )
    ///                           -----------------When to carry over a bit: call this sequence A1--
    ///     z03= ((x03 XOR y03) XOR ( (x02 AND y02) OR ( (x02 XOR y02) AND A1 ))                   )
    ///                             -----------------When to carry over a bit: call this sequence A2--
    ///     z04= ((x04 XOR y04) XOR ( (x03 AND y03) OR ( (x03 XOR yo2) AND A2 ) ))
    ///                             -----------------When to carry over a bit: call this sequence A3--
    ///     z05 is similar and so on (until the last z which is a special case).
    ///
    ///We notice from initial printing of the output that the z's do have this reperesntation.
    ///We can use this to find out which circuits were swapped with each other!
    ///
    /// Returns the names of the eight wires involved in a swap (sorted and comma seperated)
    fn evaluate(out_instruct: HashMap<&str, Circuit>, out_eval: HashMap<&str, String>) -> String {
        let mut iter = out_instruct
            .iter()
            .filter(|(key, _)| key.starts_with("z"))
            .sorted_by(|&a, &b| a.0.cmp(b.0))
            .enumerate();

        //We manually inspected the first 3 values (so we know the initial a is in z02)
        let z0 = out_eval.get(iter.next().unwrap().1.0).unwrap();
        assert_eq!(z0, "(x00 XOR y00)");

        let z1 = out_eval.get(iter.next().unwrap().1.0).unwrap();
        assert_eq!(z1, "((x01 XOR y01) XOR (x00 AND y00))");

        let mut a = "(((x00 AND y00) AND (x01 XOR y01)) OR (x01 AND y01))".to_owned();
        iter.next();

        for (index, (&z, circuit)) in iter {
            //The last z doesn't fit this pattern
            if index == 45 {
                continue;
            }

            let count_str_curr = if index < 10 {
                format!("0{index}")
            } else {
                format!("{index}")
            };

            let prev = index - 1;
            let count_str_prev = if prev < 10 {
                format!("0{prev}")
            } else {
                format!("{prev}")
            };

            let mut current_value = out_eval.get(z).unwrap().clone();

            //First we make sure A is in this String (like A1 in z03)
            if !current_value.contains(&a) {
                panic!(
                    "Bad current value: {}.  
                It is made up like so:
                in1: {}
                in2: {}
                gate type: {}
                out: {}.

                Failed to find initial A (think A1 in z03). current A is {a}
                Find the correct circuits to switch in using {:#?} ",
                    current_value, circuit.in_1, circuit.in_2, circuit.gate, circuit.out, out_eval
                )
            }

            //set up the next a
            let a_old = a.clone();

            let a_new_options = [
                format!(
                    "(({a} AND (x{count_str_prev} XOR y{count_str_prev})) OR (x{count_str_prev} AND y{count_str_prev}))"
                ),
                format!(
                    "((x{count_str_prev} AND y{count_str_prev}) OR ((x{count_str_prev} XOR y{count_str_prev}) AND {a}))"
                ),
                format!(
                    "(((x{count_str_prev} XOR y{count_str_prev}) AND {a}) OR (x{count_str_prev} AND y{count_str_prev}))"
                ),
                format!(
                    "((x{count_str_prev} AND y{count_str_prev}) OR ({a} AND (x{count_str_prev} XOR y{count_str_prev})))"
                ),
            ];

            let mut found_new = false;

            //Make sure the bigger A is in this String (like A2 in z03)
            for new_option in a_new_options {
                if current_value.contains(&new_option) {
                    found_new = true;
                    a = new_option;
                    current_value = current_value.replace(&a, "A");
                    break;
                }
            }

            if !found_new {
                panic!(
                    "Bad current value: {}. 
                Bad current value with smaller A (which was found) replaced: {} 
                It is made up like so:
                in1: {}
                in2: {}
                gate type: {}
                out: {}.

                Failed to find larger A. Current A is  {a}
                Find the correct circuits to switch in using {:#?} ",
                    current_value,
                    current_value.replace(&a_old, "A"),
                    circuit.in_1,
                    circuit.in_2,
                    circuit.gate,
                    circuit.out,
                    out_eval
                )
            }

            let other_part = format!("(x{count_str_curr} XOR y{count_str_curr})");

            if !current_value.contains(&other_part) || circuit.gate != "XOR" {
                panic!(
                    "Bad current value: {}.  
                It is made up like so:
                in1: {}
                in2: {}
                gate type: {}
                out: {}
                Find the correct circuits to switch in using {:#?} ",
                    current_value, circuit.in_1, circuit.in_2, circuit.gate, circuit.out, out_eval
                )
            }
        }

        //If we got here we got all the pairs of Circuits to swap correct.
        //We need to printout the names of the output wires of these Circuits in alphabetical order
        let mut output_wires = ["swt", "z07", "pqc", "z13", "rjm", "wsv", "bgs", "z31"];

        output_wires.sort();

        output_wires.join(",")
    }
}

///Simulates the system of gates and wires.
/// Returns the decimal number it outputs on the wires starting with z
fn solution_part1(file_path: &str) -> usize {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    let system = System::build_and_execute(data);

    system.decimal_output()
}

///Returns the names of the eight wires involved in a swap (sorted and comma seperated).
/// Swapping these 8 output wires causes the system to correctly do binary addition.
fn solution_part2(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("failed to open file");

    logic_gates::build_and_evaluate(data)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day24.txt"));
        dbg!(solution_part2("puzzle_inputs/day24.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day24example.txt");
        assert_eq!(result, 2024);
    }
}
