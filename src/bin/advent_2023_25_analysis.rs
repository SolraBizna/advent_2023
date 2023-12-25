use std::{collections::HashMap, io::Read};

use vecmap::VecSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();
    let mut nodes: HashMap<&str, VecSet<&str>> = HashMap::new();
    for line in lines {
        let (src, rest) = line.split_once(": ").unwrap();
        for dst in rest.split(' ') {
            nodes.entry(src).or_default().insert(dst);
            nodes.entry(dst).or_default().insert(src);
        }
    }
    for node in nodes.keys() {
        println!("[{node}]");
    }
    for (src, dsts) in nodes.iter() {
        for dst in dsts.iter() {
            if src > dst {
                println!("{src} 1--1 {dst}");
            }
        }
    }
}
