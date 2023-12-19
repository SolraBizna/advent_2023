use std::{
    collections::{HashMap, VecDeque},
    ops::{Add, Range},
};

#[derive(Debug, Clone)]
struct Part<T: Clone> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone> Part<T> {
    fn get(&self, key: &str) -> &T {
        match key {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!(),
        }
    }
    fn replacing(&self, key: &str, value: T) -> Part<T> {
        let mut value = Some(value);
        Part {
            x: if key == "x" {
                value.take().unwrap()
            } else {
                self.x.clone()
            },
            m: if key == "m" {
                value.take().unwrap()
            } else {
                self.m.clone()
            },
            a: if key == "a" {
                value.take().unwrap()
            } else {
                self.a.clone()
            },
            s: if key == "s" {
                value.take().unwrap()
            } else {
                self.s.clone()
            },
        }
    }
}

impl<T: Clone + Copy + Add<T, Output = T>> Part<T> {
    fn sum(&self) -> T {
        self.x + self.m + self.a + self.s
    }
}

impl Part<Range<i32>> {
    fn len(&self) -> u64 {
        self.x.len() as u64
            * self.m.len() as u64
            * self.a.len() as u64
            * self.s.len() as u64
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn new() -> Self {
        Part {
            x: 0..0,
            m: 0..0,
            a: 0..0,
            s: 0..0,
        }
    }
}

impl<P: Clone, S: AsRef<str>> FromIterator<(S, P)> for Part<P> {
    fn from_iter<T: IntoIterator<Item = (S, P)>>(iter: T) -> Self {
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;
        for (key, value) in iter {
            match key.as_ref() {
                "x" => x = Some(value),
                "m" => m = Some(value),
                "a" => a = Some(value),
                "s" => s = Some(value),
                _ => panic!(),
            }
        }
        Part {
            x: x.unwrap(),
            m: m.unwrap(),
            a: a.unwrap(),
            s: s.unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ComparisonOperator {
    LessThan,
    GreaterThan,
}

impl ComparisonOperator {
    fn from_char(ch: char) -> ComparisonOperator {
        match ch {
            '<' => ComparisonOperator::LessThan,
            '>' => ComparisonOperator::GreaterThan,
            x => panic!("Unknown comparison operator {x:?}"),
        }
    }
    fn evaluate(&self, input1: i32, input2: i32) -> bool {
        match self {
            ComparisonOperator::LessThan => input1 < input2,
            ComparisonOperator::GreaterThan => input1 > input2,
        }
    }
    /// Returns: (unmatched, matched)
    fn split(
        &self,
        inputs: &Range<i32>,
        input2: i32,
    ) -> (Range<i32>, Range<i32>) {
        match self {
            ComparisonOperator::LessThan => {
                let (less, great) = split_less_than(inputs, input2);
                (great, less)
            }
            ComparisonOperator::GreaterThan => {
                split_less_than_or_equal(inputs, input2)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CommandResult<'a> {
    Reject,
    Accept,
    Destination(&'a str),
}

#[derive(Debug, Clone)]
enum Command {
    Conditional {
        input: String,
        value: i32,
        comparison_operator: ComparisonOperator,
        destination: String,
    },
    Unconditional {
        destination: String,
    },
}

impl Command {
    fn from_str(command_string: &str) -> Command {
        match command_string.find(|x: char| !x.is_ascii_alphanumeric()) {
            None => Command::Unconditional {
                destination: command_string.to_string(),
            },
            Some(split_point) => {
                let input = command_string[..split_point].to_string();
                let comparison_operator = ComparisonOperator::from_char(
                    command_string[split_point..].chars().next().unwrap(),
                );
                let rest = &command_string[split_point + 1..];
                let (value, destination) = rest.split_once(':').unwrap();
                Command::Conditional {
                    input,
                    value: value.parse().unwrap(),
                    comparison_operator,
                    destination: destination.to_string(),
                }
            }
        }
    }
    fn execute(&'_ self, part: &Part<i32>) -> Option<CommandResult<'_>> {
        match self {
            Command::Conditional {
                input,
                value,
                comparison_operator,
                destination,
            } => {
                if comparison_operator.evaluate(*part.get(input), *value) {
                    if destination == "A" {
                        Some(CommandResult::Accept)
                    } else if destination == "R" {
                        Some(CommandResult::Reject)
                    } else {
                        Some(CommandResult::Destination(destination))
                    }
                } else {
                    None
                }
            }
            Command::Unconditional { destination } => {
                if destination == "A" {
                    Some(CommandResult::Accept)
                } else if destination == "R" {
                    Some(CommandResult::Reject)
                } else {
                    Some(CommandResult::Destination(destination))
                }
            }
        }
    }
    fn wide_execute(
        &'_ self,
        part: &mut Part<Range<i32>>,
    ) -> Option<(Part<Range<i32>>, CommandResult<'_>)> {
        match self {
            Command::Conditional {
                input,
                value,
                comparison_operator,
                destination,
            } => {
                let (unmatched_range, matched_range) =
                    comparison_operator.split(part.get(input), *value);
                let result = if !matched_range.is_empty() {
                    if destination == "A" {
                        Some((matched_range, CommandResult::Accept))
                    } else if destination == "R" {
                        Some((matched_range, CommandResult::Reject))
                    } else {
                        Some((
                            matched_range,
                            CommandResult::Destination(destination),
                        ))
                    }
                } else {
                    None
                };
                let ret = result.map(|(matched_range, result)| {
                    (part.replacing(input, matched_range), result)
                });
                *part = part.replacing(input, unmatched_range);
                ret
            }
            Command::Unconditional { destination } => {
                let result = if destination == "A" {
                    CommandResult::Accept
                } else if destination == "R" {
                    CommandResult::Reject
                } else {
                    CommandResult::Destination(destination)
                };
                let mut ret = Part::new();
                std::mem::swap(part, &mut ret);
                Some((ret, result))
            }
        }
    }
    fn perform_workflow<'a>(
        part: &Part<i32>,
        workflow: &'a [Command],
    ) -> CommandResult<'a> {
        workflow
            .iter()
            .filter_map(|command| command.execute(part))
            .next()
            .unwrap()
    }
    fn is_part_accepted(
        part: &Part<i32>,
        workflows: &HashMap<String, Vec<Command>>,
    ) -> bool {
        let mut current_workflow = "in";
        loop {
            let workflow = workflows.get(current_workflow).unwrap();
            match Command::perform_workflow(part, workflow) {
                CommandResult::Reject => return false,
                CommandResult::Accept => return true,
                CommandResult::Destination(destination) => {
                    current_workflow = destination
                }
            }
        }
    }
    // see how the above is all nicely factored? yeah, not doing that this time
    // (apparently)
    fn count_accepted_parts(
        parts: Part<Range<i32>>,
        workflows: &HashMap<String, Vec<Command>>,
    ) -> u64 {
        let mut queue = VecDeque::with_capacity(1024);
        queue.push_front(("in", parts));
        let mut ret = 0;
        while let Some((current_workflow, parts)) = queue.pop_back() {
            let workflow = workflows.get(current_workflow).unwrap();
            let mut parts = parts;
            let ret = &mut ret;
            let queue = &mut queue;
            workflow.iter().for_each(move |command| {
                if parts.is_empty() {
                    return;
                }
                let r#match = command.wide_execute(&mut parts);
                if let Some((matched, result)) = r#match {
                    match result {
                        CommandResult::Reject => (),
                        CommandResult::Accept => *ret += matched.len(),
                        CommandResult::Destination(destination) => {
                            queue.push_front((destination, matched));
                        }
                    }
                }
            });
        }
        ret
    }
}

fn split_less_than(
    range: &Range<i32>,
    target: i32,
) -> (Range<i32>, Range<i32>) {
    if range.end < target {
        (range.clone(), 0..0)
    } else if range.start >= target {
        (0..0, range.clone())
    } else {
        (range.start..target, target..range.end)
    }
}

fn split_less_than_or_equal(
    range: &Range<i32>,
    target: i32,
) -> (Range<i32>, Range<i32>) {
    split_less_than(range, target + 1)
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut workflows: HashMap<String, Vec<Command>> = HashMap::new();
    // parse the workflows
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (name, rest) = line.split_once('{').unwrap();
        let rest = rest.strip_suffix('}').unwrap();
        workflows.insert(
            name.to_string(),
            rest.split(',').map(Command::from_str).collect(),
        );
    }
    // parse the inputs
    let parts: Vec<Part<i32>> = lines
        .map(|line| {
            line.strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|kv| {
                    let (k, v) = kv.split_once('=').unwrap();
                    (k.to_string(), v.parse().unwrap())
                })
                .collect()
        })
        .collect();
    println!(
        "Puzzle 1 solution: {}",
        parts
            .iter()
            .filter(|part| Command::is_part_accepted(part, &workflows))
            .map(|part| { part.sum() })
            .sum::<i32>()
    );
    // Part 2 is a challenge worthy of a Klingon warrior!
    println!(
        "Puzzle 2 solution: {}",
        Command::count_accepted_parts(
            Part {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            },
            &workflows
        ),
    );
}
