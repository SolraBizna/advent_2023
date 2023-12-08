use std::collections::HashMap;

use num::Integer;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    pub fn follow<'a>(
        &self,
        left_choice: &'a str,
        right_choice: &'a str,
    ) -> &'a str {
        match self {
            Dir::Left => left_choice,
            Dir::Right => right_choice,
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let instructions: Vec<Dir> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| {
            if x == 'L' {
                Dir::Left
            } else if x == 'R' {
                Dir::Right
            } else {
                panic!()
            }
        })
        .collect();
    let instructions = instructions.into_iter().cycle();
    assert_eq!(lines.next().unwrap(), "");
    let graph: HashMap<String, (String, String)> = lines
        .map(|x| {
            let (lhs, rhs) = x.split_once(" = ").unwrap();
            let (left, right) = rhs
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap(); // holy unwrap batman!
            (lhs.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();
    if graph.contains_key("AAA") {
        let mut location = "AAA";
        let mut steps = 0;
        for instruction in instructions.clone() {
            let here = graph.get(location).unwrap();
            location = instruction.follow(&here.0, &here.1);
            steps += 1;
            if location == "ZZZ" {
                break;
            }
        }
        println!("Puzzle 1 solution: {steps}");
    }
    let jumps_and_cycles: Vec<(u64, u64)> = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(String::as_str)
        .map(|start_location| {
            let mut instructions = instructions.clone();
            let mut steps = 0;
            let mut location = start_location;
            for instruction in &mut instructions {
                let here = graph.get(location).unwrap();
                location = instruction.follow(&here.0, &here.1);
                steps += 1;
                if location.ends_with('Z') {
                    break;
                }
            }
            let first_length = steps;
            for instruction in instructions {
                let here = graph.get(location).unwrap();
                location = instruction.follow(&here.0, &here.1);
                steps += 1;
                if location.ends_with('Z') {
                    break;
                }
            }
            let cycle_length = steps - first_length;
            assert_eq!(first_length, cycle_length); // holds for my input!
            (first_length, cycle_length)
        })
        .collect();
    println!(
        "Part 2 solution: {}",
        jumps_and_cycles.into_iter().fold(
            1,
            |accumulator, (_, cycle_length)| {
                accumulator.lcm(&cycle_length)
            }
        )
    );
}
