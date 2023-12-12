use std::str::FromStr;

use rayon::prelude::*;

#[repr(u8)] // premature optimization is the hobgoblin of little minds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl State {
    fn from_char(ch: char) -> State {
        match ch {
            '.' => State::Operational,
            '#' => State::Damaged,
            '?' => State::Unknown,
            x => panic!("unknown spring state {x:?}"),
        }
    }
}

struct PartialSpringRow<'a> {
    states: &'a [State],
    picross_hint: &'a [usize],
}

impl PartialSpringRow<'_> {
    fn hint_can_hold_at(&self, n: usize, hint: usize) -> bool {
        self.states[n..n + hint]
            .iter()
            .all(|x| *x == State::Damaged || *x == State::Unknown)
            && (n + hint == self.states.len()
                || self.states[n + hint] == State::Operational
                || self.states[n + hint] == State::Unknown)
    }
    fn elegant_arrogant_smart_iteration(&mut self) -> usize {
        // If our picross hint has been consumed...
        if self.picross_hint.is_empty() {
            if self.states.iter().all(|x| *x != State::Damaged) {
                // No damaged springs left to report. We could exist.
                return 1;
            } else {
                // At least one damaged spring left, but no numbers left in our
                // picross hint. We couldn't exist.
                return 0;
            }
        }
        // Strip off any leading operational springs.
        while let Some(x) = self.states.strip_prefix(&[State::Operational]) {
            self.states = x;
        }
        // Make sure that we can even FIT a proper solution.
        let total_hint_requirement =
            self.picross_hint.iter().map(|x| *x + 1).sum::<usize>() - 1;
        if self.states.len() < total_hint_requirement {
            return 0; // Uh, no. Not possible from here.
        }
        // We're checking the next picross hint.
        let next_hint = self.picross_hint[0];
        if next_hint > self.states.len() {
            // Hint requires more remaining springs than we ... are.
            return 0;
        }
        let mut ret = 0;
        // Let a little of the brutishness back in.
        for n in 0..=self.states.len() - total_hint_requirement {
            if self.hint_can_hold_at(n, next_hint) {
                /*if n == self.states.len() - next_hint {
                    if self.picross_hint.len() == 1 {
                        // The final hint fits in the final slot.
                        ret += 1;
                    } else {
                        // This would require more slots after...
                    }
                    break;
                }*/
                // Try this spot.
                ret += PartialSpringRow {
                    states: &self.states
                        [(n + next_hint + 1).min(self.states.len())..],
                    picross_hint: &self.picross_hint[1..],
                }
                .elegant_arrogant_smart_iteration()
            }
            if self.states[n] == State::Damaged {
                // Any solution anchored after this would not match this
                // state.
                break;
            }
        }
        ret
    }
}

struct SpringRow {
    states: Vec<State>,
    picross_hint: Vec<usize>,
}

impl FromStr for SpringRow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<SpringRow> {
        let (left, right) = s.split_once(' ').unwrap();
        let states = left.chars().map(State::from_char).collect();
        let picross_hint =
            right.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(SpringRow {
            states,
            picross_hint,
        })
    }
}

fn evaluate_condition(states: &[State]) -> Vec<usize> {
    // overshoot and you'll hit the moon!
    let mut ret = Vec::with_capacity(states.len());
    let mut am_damaged = false;
    // this will be easier to read as a for loop :(
    for spring in states.iter() {
        match spring {
            State::Operational => {
                am_damaged = false;
            }
            State::Damaged => {
                if !am_damaged {
                    ret.push(1);
                } else {
                    *ret.last_mut().unwrap() += 1;
                }
                am_damaged = true;
            }
            State::Unknown => todo!(),
        }
    }
    ret
}

impl SpringRow {
    fn brute_force_count_permutations(&self) -> usize {
        let mut hypothetical: Vec<State> = self
            .states
            .iter()
            .map(|x| match x {
                State::Operational => State::Operational,
                State::Damaged => State::Damaged,
                State::Unknown => State::Operational,
            })
            .collect();
        let mut ret = 0;
        loop {
            if evaluate_condition(&hypothetical) == self.picross_hint {
                ret += 1;
            }
            let mut stop = true;
            for n in 0..hypothetical.len() {
                if self.states[n] == State::Unknown {
                    match hypothetical[n] {
                        State::Operational => {
                            hypothetical[n] = State::Damaged;
                            stop = false;
                            break;
                        }
                        State::Damaged => {
                            hypothetical[n] = State::Operational;
                            // do not break
                        }
                        State::Unknown => unreachable!(),
                    }
                }
            }
            if stop {
                break;
            }
        }
        ret
    }
    fn elegant_arrogant_smart_count_permutations(&self) -> usize {
        (PartialSpringRow {
            states: &self.states,
            picross_hint: &self.picross_hint,
        })
        .elegant_arrogant_smart_iteration()
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let rows: Vec<SpringRow> = lines.map(|x| x.parse().unwrap()).collect();
    println!(
        "Puzzle 1 answer: {}",
        rows.par_iter()
            .map(|x| x.elegant_arrogant_smart_count_permutations())
            .sum::<usize>(),
    );
    let rowsrowsrowsrowsrows: Vec<SpringRow> = rows
        .iter()
        .map(|source| {
            let mut states = Vec::with_capacity(source.states.len() * 5 + 4);
            let mut picross_hint =
                Vec::with_capacity(source.picross_hint.len() * 5);
            for n in 0..5 {
                states.extend(source.states.iter());
                picross_hint.extend(source.picross_hint.iter());
                if n != 4 {
                    states.push(State::Unknown);
                }
            }
            SpringRow {
                states,
                picross_hint,
            }
        })
        .collect();
    println!(
        "\nPuzzle 2 answer: {}",
        rowsrowsrowsrowsrows
            .par_iter()
            .map(|x| {
                eprint!(".");
                x.elegant_arrogant_smart_count_permutations()
            })
            .sum::<usize>(),
    );
}

#[test]
fn test_elegant_sample1() {
    let row: SpringRow = "???.### 1,1,3".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 1);
}

#[test]
fn test_elegant_sample1x2() {
    let row: SpringRow = "???.###????.### 1,1,3,1,1,3".parse().unwrap();
    assert_eq!(
        row.elegant_arrogant_smart_count_permutations(),
        row.brute_force_count_permutations()
    );
}

#[test]
fn test_elegant_sample1_1x5() {
    let row: SpringRow =
        "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
            .parse()
            .unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 1);
}

#[test]
fn test_elegant_sample1_2x5() {
    let row: SpringRow = ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 16384);
}

#[test]
fn test_elegant_sample1_3x2() {
    let row: SpringRow = "?#?#?#?#?#?#?#???#?#?#?#?#?#?#?? 1,3,1,6,1,3,1,6"
        .parse()
        .unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 1);
}

#[test]
fn test_elegant_sample1_3x5() {
    let row: SpringRow = "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#? 1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 1);
}

#[test]
fn test_elegant_sample1_4x5() {
    let row: SpringRow = "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#... 4,1,1,4,1,1,4,1,1,4,1,1,4,1,1".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 16);
}

#[test]
fn test_elegant_sample1_5x5() {
    let row: SpringRow = "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####. 1,6,5,1,6,5,1,6,5,1,6,5,1,6,5".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 2500);
}

#[test]
fn test_elegant_sample1_6x5() {
    let row: SpringRow = "?###??????????###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 506250);
}

#[test]
fn is_this_the_edge_case() {
    let row: SpringRow = "#?#?#?#?? 1,6".parse().unwrap();
    assert_eq!(row.elegant_arrogant_smart_count_permutations(), 1);
}
