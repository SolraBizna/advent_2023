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

/// Returns: (ending location, number of steps taken)
fn follow_until_condition<'a>(
    starting_location: &'a str,
    instructions: impl Iterator<Item = Dir>,
    graph: &'a HashMap<String, (String, String)>,
    finish_condition: impl Fn(&'a str) -> bool,
) -> (&'a str, u64) {
    let mut location = starting_location;
    let mut steps = 0;
    for instruction in instructions {
        let here = graph.get(location).unwrap();
        location = instruction.follow(&here.0, &here.1);
        steps += 1;
        if finish_condition(location) {
            break;
        }
    }
    (location, steps)
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
        let (ending_location, steps) =
            follow_until_condition("AAA", instructions.clone(), &graph, |x| {
                x == "ZZZ"
            });
        assert_eq!(ending_location, "ZZZ");
        println!("Puzzle 1 solution: {steps}");
    }
    let cycle_lengths: Vec<u64> = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(String::as_str)
        .map(|start_location| {
            let mut instructions = instructions.clone();
            let (unique_end, unique_steps) = follow_until_condition(
                start_location,
                &mut instructions,
                &graph,
                |x| x.ends_with('Z'),
            );
            let (_cycle_end, cycle_steps) = follow_until_condition(
                unique_end,
                &mut instructions,
                &graph,
                |x| x.ends_with('Z'),
            );
            assert_eq!(unique_steps, cycle_steps); // holds for my input!
            cycle_steps
        })
        .collect();
    println!(
        "Part 2 solution: {}",
        cycle_lengths
            .into_iter()
            .fold(1, |accumulator, cycle_length| {
                accumulator.lcm(&cycle_length)
            })
    );
}
