use std::{io::Write, ops::Range};

use rayon::prelude::*;

struct Supermap {
    from: String,
    to: String,
    mappings: Vec<(u64, Range<u64>)>,
}

impl Supermap {
    fn add_mapping(&mut self, dst_start: u64, src_start: u64, len: u64) {
        self.mappings.push((dst_start, src_start..src_start + len));
    }
    pub fn read_map(
        lines: &mut impl Iterator<Item = String>,
    ) -> Option<Supermap> {
        let line = lines.next()?;
        let line = line.strip_suffix(" map:").unwrap();
        let (from, to) = line.split_once("-to-").unwrap();
        let from = from.to_string();
        let to = to.to_string();
        let mut ret = Supermap {
            from,
            to,
            mappings: vec![],
        };
        for line in lines {
            if line.is_empty() {
                // We will also (implicitly) break if we run out of lines.
                break;
            }
            let numbers: Vec<u64> =
                line.split(' ').map(|x| x.parse().unwrap()).collect();
            ret.add_mapping(numbers[0], numbers[1], numbers[2]);
        }
        ret.mappings
            .sort_by_key(|(_dst_start, src_range)| src_range.start);
        Some(ret)
    }
    pub fn remap(&self, input: u64) -> u64 {
        let success = match self
            .mappings
            .binary_search_by_key(&input, |(_dst_start, src_range)| {
                src_range.start
            }) {
            Ok(index) => {
                // Exact match on the start of a range
                let (dst_start, src_range) = &self.mappings[index];
                Some((dst_start, src_range))
            }
            Err(index) => {
                // Might be out of range, might be the index of a range AFTER
                // us, might be a range before us
                if index > self.mappings.len() || index == 0 {
                    None
                } else {
                    let (dst_start, src_range) = &self.mappings[index - 1];
                    if src_range.contains(&input) {
                        Some((dst_start, src_range))
                    } else {
                        None
                    }
                }
            }
        };
        match success {
            Some((dst_start, src_range)) => {
                debug_assert!(src_range.contains(&input));
                input - src_range.start + dst_start
            }
            None => input,
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let seedline = lines.next().unwrap();
    let (_, seedlist) = seedline.split_once(": ").unwrap();
    let seeds: Vec<u64> = seedlist
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    assert_eq!(lines.next().unwrap().as_str(), "");
    let maps: Vec<Supermap> =
        std::iter::from_fn(|| Supermap::read_map(&mut lines)).collect();
    assert_eq!(maps[0].from, "seed");
    for n in 0..maps.len() - 1 {
        assert_eq!(&maps[n].to, &maps[n + 1].from);
    }
    assert_eq!(maps.last().unwrap().to, "location");
    let mut lowest_location = u64::MAX;
    for seed in seeds.iter() {
        let location = maps
            .iter()
            .fold(*seed, |input, mapping| mapping.remap(input));
        if location < lowest_location {
            lowest_location = location;
        }
    }
    println!("Puzzle 1 answer: {lowest_location}");
    let mut all_chunks = vec![];
    // How many seeds to distribute to each core.
    const CHUNK_SIZE: u64 = 10_000_000;
    let mut total = 0;
    for chunk in seeds.chunks_exact(2) {
        let start = chunk[0];
        let length = chunk[1];
        for staato in (start..start + length).step_by(CHUNK_SIZE as usize) {
            all_chunks.push(staato..(staato + CHUNK_SIZE).min(start + length));
        }
        total += length;
    }
    println!("{total}");
    println!("Number of chunks to try: {}", all_chunks.len());
    for _ in 0..all_chunks.len() {
        print!("░")
    }
    print!("\r");
    let lowest_location = all_chunks
        .par_iter()
        .map(|range| {
            print!("▓");
            let _ = std::io::stdout().flush();
            range.clone().fold(u64::MAX, |lowest_location, seed| {
                let location = maps
                    .iter()
                    .fold(seed, |input, mapping| mapping.remap(input));
                location.min(lowest_location)
            })
        })
        .reduce(|| u64::MAX, u64::min);
    println!("\nPuzzle 2 answer: {lowest_location}");
}
