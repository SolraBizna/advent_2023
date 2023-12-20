use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use compact_str::{CompactString, ToCompactString};
use num::Integer;

trait Module: Debug + Any {
    /// Called during setup time to inform this module that an input is
    /// connected to it.
    fn connect_input(&mut self, _input_name: CompactString) {}
    /// Called during simulation time to indicate that the module has received
    /// an input pulse from one of its inputs. Returns `None` if it emits no
    /// pulse in response, `Some(false)` if it emits a low pulse, or
    /// `Some(true)` if it emits a high pulse.
    fn receive_pulse(
        &mut self,
        input_name: &str,
        input_high: bool,
    ) -> Option<bool>;
    /// Create a boxed clone of ourselves. (Ick!)
    fn boxed_clone(&self) -> Box<dyn Module>;
    /// Hack! Only works on NandGates!
    fn get_inputs(&self) -> Vec<&str> {
        panic!()
    }
}

#[derive(Debug, Clone)]
struct Broadcaster;
impl Module for Broadcaster {
    fn receive_pulse(
        &mut self,
        _input_name: &str,
        input_high: bool,
    ) -> Option<bool> {
        Some(input_high)
    }
    fn boxed_clone(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}

#[derive(Default, Debug, Clone)]
struct FlipFlop {
    current_state: bool,
}

impl Module for FlipFlop {
    fn receive_pulse(
        &mut self,
        _input_name: &str,
        input_high: bool,
    ) -> Option<bool> {
        if !input_high {
            self.current_state = !self.current_state;
            Some(self.current_state)
        } else {
            None
        }
    }
    fn boxed_clone(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}

#[derive(Default, Debug, Clone)]
struct NandGate {
    input_states: HashMap<CompactString, bool>,
}

impl Module for NandGate {
    fn receive_pulse(
        &mut self,
        input_name: &str,
        input_high: bool,
    ) -> Option<bool> {
        *self.input_states.get_mut(input_name).unwrap() = input_high;
        Some(!self.input_states.values().all(|x| *x))
    }
    fn connect_input(&mut self, input_name: CompactString) {
        self.input_states.insert(input_name, false);
    }
    fn boxed_clone(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
    fn get_inputs(&self) -> Vec<&str> {
        self.input_states
            .keys()
            .map(CompactString::as_str)
            .collect()
    }
}

fn simulate<F: FnMut(u64, &str, &str, bool) -> Option<R>, R>(
    modules: &HashMap<CompactString, (Box<dyn Module>, Vec<CompactString>)>,
    mut terminator: F,
) -> R {
    let mut modules: HashMap<
        CompactString,
        (Box<dyn Module>, Vec<CompactString>),
    > = modules
        .iter()
        .map(|(k, (m, v))| (k.clone(), (m.boxed_clone(), v.clone())))
        .collect();
    let mut queue = VecDeque::with_capacity(1024);
    for num_presses in 1.. {
        queue.push_back((
            "".to_compact_string(),
            "broadcaster".to_compact_string(),
            false,
        ));
        while let Some((source, destination, pulse)) = queue.pop_front() {
            if let Some(ret) =
                terminator(num_presses, source.as_str(), &destination, pulse)
            {
                return ret;
            }
            //println!("({source:?}, {destination:?}, {pulse:?})");
            let Some((module, destinations)) = modules.get_mut(&destination)
            else {
                continue;
            };
            match module.receive_pulse(source.as_str(), pulse) {
                None => (),
                Some(pulse) => {
                    let source = destination;
                    for destination in destinations.iter() {
                        queue.push_back((
                            source.to_compact_string(),
                            destination.to_compact_string(),
                            pulse,
                        ));
                    }
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut modules: HashMap<
        CompactString,
        (Box<dyn Module>, Vec<CompactString>),
    > = lines
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let (name, module): (&str, Box<dyn Module>) =
                if let Some(name) = name.strip_prefix('%') {
                    (name, Box::<FlipFlop>::default())
                } else if let Some(name) = name.strip_prefix('&') {
                    (name, Box::<NandGate>::default())
                } else {
                    assert_eq!(name, "broadcaster");
                    (name, Box::new(Broadcaster))
                };
            (
                name.to_compact_string(),
                (
                    module,
                    destinations
                        .split(", ")
                        .map(|x| x.to_compact_string())
                        .collect(),
                ),
            )
        })
        .collect();
    // plumb the inputs -_-
    let mut destination = None;
    let ios: Vec<(CompactString, CompactString)> = modules
        .iter()
        .flat_map(|(name, (_, destinations))| {
            destinations.iter().map(|dest| (name.clone(), dest.clone()))
        })
        .collect();
    for (from, to) in ios.into_iter() {
        if to == "rx" {
            assert!(destination.is_none(), "More than one input to rx!");
            destination = Some(from.to_compact_string());
        }
        if let Some((module, _destinations)) = modules.get_mut(&to) {
            module.connect_input(from);
        }
    }
    // okay, let's simulate!
    let mut num_low_pulses = 0;
    let mut num_high_pulses = 0;
    simulate(&modules, |num_pulses, _source, _destination, pulse| {
        if num_pulses == 1001 {
            dbg!(num_low_pulses);
            dbg!(num_high_pulses);
            println!("Part 1 answer: {}", num_low_pulses * num_high_pulses);
            Some(())
        } else {
            if pulse {
                num_high_pulses += 1;
            } else {
                num_low_pulses += 1;
            }
            None
        }
    });
    // That was part 1. Part 2 is going to be... odder.
    if let Some(destination) = destination {
        let (dest, _) = modules.get(&destination).unwrap();
        let dest_inputs = dest.get_inputs();
        let dest_periods: Vec<u64> = dest_inputs
            .iter()
            .copied()
            .map(|parent| {
                let mut first_hit = None;
                let second_hit = simulate(
                    &modules,
                    |num_pulses, _source, destination, pulse| {
                        if pulse || destination != parent {
                            return None;
                        }
                        if first_hit.is_none() {
                            first_hit = Some(num_pulses);
                            return None;
                        }
                        Some(num_pulses)
                    },
                );
                let first_hit = first_hit.unwrap();
                assert_eq!(second_hit, first_hit * 2);
                first_hit
            })
            .collect();
        println!(
            "Part 2 answer: {}",
            dest_periods.into_iter().fold(1, |a, b| a.lcm(&b))
        );
    }
}
