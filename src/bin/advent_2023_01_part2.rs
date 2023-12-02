const DIGIT_MAP: &[(&[u8], i32)] = &[
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
];

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lines()
            .map(|x| {
                let x = x.unwrap();
                let mut rest = x.as_bytes();
                let mut first = None;
                let mut last = None;
                while !rest.is_empty() {
                    for (prefix, value) in DIGIT_MAP.iter() {
                        if rest.starts_with(prefix) {
                            if first.is_none() {
                                first = Some(value);
                            }
                            last = Some(value);
                            // do not strip the prefix, just let the next byte
                            // get stripped... overlapping digits may count!
                            break;
                        }
                    }
                    rest = &rest[1..];
                }
                first.unwrap() * 10 + last.unwrap()
            })
            .sum::<i32>()
    );
}
