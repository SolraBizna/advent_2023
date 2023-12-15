fn christmahash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0, |a, x| a.wrapping_add(*x).wrapping_mul(17))
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let line = lines.next().unwrap();
    println!(
        "Puzzle 1 answer: {}",
        line.split(',').map(|x| christmahash(x) as u32).sum::<u32>()
    );
    let mut buckets: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    for instruction in line.split(',') {
        let split_point = instruction.find(|x| x == '-' || x == '=').unwrap();
        let label = &instruction[..split_point];
        let length = &instruction[split_point + 1..];
        let command = instruction.as_bytes()[split_point];
        let hash = christmahash(label) as usize;
        match command {
            b'-' => {
                // Remove!
                buckets[hash].retain(|(k, _)| *k != label);
                assert_eq!(length, "");
            }
            b'=' => {
                // Insert!
                let length = length.parse::<u8>().unwrap();
                assert!((1..=9).contains(&length));
                let mut found = false;
                for (k, v) in buckets[hash].iter_mut() {
                    if *k == label {
                        *v = length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    buckets[hash].push((label, length));
                }
            }
            _ => unreachable!(),
        }
    }
    for (n, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            println!("buckets[{n}] = {:?}", bucket);
        }
    }
    println!(
        "Puzzle 2 answer: {}",
        buckets
            .iter()
            .enumerate()
            .map(|(i, bucket)| {
                (i as u32 + 1)
                    * bucket
                        .iter()
                        .enumerate()
                        .map(|(j, (_, length))| {
                            *length as u32 * (j as u32 + 1)
                        })
                        .sum::<u32>()
            })
            .sum::<u32>()
    );
}
