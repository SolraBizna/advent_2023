use std::{
    collections::{HashMap, HashSet},
    io::Read,
    sync::Arc,
};

use vecmap::VecSet;

fn test_partition<'a>(
    nodes: &HashMap<&'a str, VecSet<&'a str>>,
    red_seen: &mut HashSet<&'a str>,
    black_seen: &mut HashSet<&'a str>,
    red_buf: &mut Vec<&'a str>,
    black_buf: &mut Vec<&'a str>,
    red_seed: &'a str,
    black_seed: &'a str,
) {
    red_seen.clear();
    black_seen.clear();
    red_buf.clear();
    red_buf.push(red_seed);
    black_buf.clear();
    black_buf.push(black_seed);
    loop {
        let red_next = match red_buf.pop() {
            Some(x) => x,
            None => {
                println!(
                    "Red wins! Puzzle answer: {}",
                    red_seen.len() * (nodes.len() - red_seen.len())
                );
                std::process::exit(0);
            }
        };
        let black_next = match black_buf.pop() {
            Some(x) => x,
            None => {
                println!(
                    "Black wins! Puzzle answer: {}",
                    black_seen.len() * (nodes.len() - black_seen.len())
                );
                std::process::exit(0);
            }
        };
        if red_seen.contains(black_next) || black_seen.contains(red_next) {
            // they met
            return;
        }
        black_seen.insert(black_next);
        red_seen.insert(red_next);
        for つぎ in nodes.get(black_next).unwrap() {
            if black_seen.contains(つぎ) {
                continue;
            }
            black_buf.push(つぎ);
        }
        for つぎ in nodes.get(red_next).unwrap() {
            if red_seen.contains(つぎ) {
                continue;
            }
            red_buf.push(つぎ);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = Box::new(args);
    let args = args.leak();
    let forced_one = args.get(1);
    let forced_two = args.get(2);
    let forced_three = args.get(3);
    dbg!(forced_one, forced_two, forced_three);
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = Box::new(input);
    let input = input.leak();
    let lines = input.lines();
    let mut nodes: HashMap<&str, VecSet<&str>> = HashMap::new();
    for line in lines {
        let (src, rest) = line.split_once(": ").unwrap();
        for dst in rest.split(' ') {
            nodes.entry(src).or_default().insert(dst);
            nodes.entry(dst).or_default().insert(src);
        }
    }
    let every_connection: Arc<Vec<(&str, &str)>> = Arc::new(
        nodes
            .iter()
            .flat_map(|(src, dsts)| {
                dsts.iter().filter_map(move |dst| {
                    if src < dst {
                        Some((*src, *dst))
                    } else {
                        None
                    }
                })
            })
            .collect(),
    );
    let join_handles: Vec<_> = (0..num_cpus::get())
        .map(|cpu_index| {
            let mut nodes = nodes.clone();
            let every_connection = every_connection.clone();
            let mut red_seen = HashSet::with_capacity(nodes.len());
            let mut black_seen = HashSet::with_capacity(nodes.len());
            let mut red_queue = Vec::with_capacity(nodes.len());
            let mut black_queue = Vec::with_capacity(nodes.len());
            std::thread::spawn(move || {
                for i in (0..every_connection.len() - 2)
                    .skip(cpu_index)
                    .step_by(num_cpus::get())
                {
                    let (from1, to1) = &every_connection[i];
                    if let Some(forced) = forced_three {
                        if from1 != forced && to1 != forced {
                            continue;
                        }
                    }
                    nodes.get_mut(from1).unwrap().remove(to1);
                    nodes.get_mut(to1).unwrap().remove(from1);
                    for j in i + 1..every_connection.len() - 1 {
                        let (from2, to2) = &every_connection[j];
                        if let Some(forced) = forced_two {
                            if from2 != forced && to2 != forced {
                                continue;
                            }
                        }
                        nodes.get_mut(from2).unwrap().remove(to2);
                        nodes.get_mut(to2).unwrap().remove(from2);
                        for k in j + 1..every_connection.len() {
                            let (from3, to3) = &every_connection[k];
                            if let Some(forced) = forced_one {
                                if from3 != forced && to3 != forced {
                                    continue;
                                }
                            }
                            nodes.get_mut(from3).unwrap().remove(to3);
                            nodes.get_mut(to3).unwrap().remove(from3);
                            test_partition(
                                &nodes,
                                &mut red_seen,
                                &mut black_seen,
                                &mut red_queue,
                                &mut black_queue,
                                from3,
                                to3,
                            );
                            nodes.get_mut(from3).unwrap().insert(to3);
                            nodes.get_mut(to3).unwrap().insert(from3);
                        }
                        nodes.get_mut(from2).unwrap().insert(to2);
                        nodes.get_mut(to2).unwrap().insert(from2);
                    }
                    nodes.get_mut(from1).unwrap().insert(to1);
                    nodes.get_mut(to1).unwrap().insert(from1);
                }
            })
        })
        .collect();
    join_handles.into_iter().for_each(|x| {
        let _ = x.join();
    });
    println!("No solution? :(");
    std::process::exit(1);
}
