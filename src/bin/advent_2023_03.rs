use std::ops::Range;

struct Part {
    x: Range<usize>,
    y: usize,
    number: u32,
    is_adjacent: bool,
}

struct Symbol {
    x: usize,
    y: usize,
    symbol: char,
    adjacent_part_numbers: Vec<u32>,
}

const GEAR_SYMBOL: char = '*';

fn main() {
    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (y, line) in std::io::stdin().lines().map(|x| x.unwrap()).enumerate() {
        for (x, character) in line.chars().enumerate() {
            if let Some(digit) = character.to_digit(10) {
                if let Some(last_part) = parts.last_mut() {
                    if last_part.x.end == x {
                        // This is a continuation of an already-seen part
                        // number. Extend it.
                        last_part.x.end += 1;
                        last_part.number = last_part.number * 10 + digit;
                        continue;
                    }
                }
                parts.push(Part {
                    x: x..x + 1,
                    y,
                    number: digit,
                    is_adjacent: false,
                });
            } else if character == '.' {
                // do nothing
            } else {
                symbols.push(Symbol {
                    x,
                    y,
                    symbol: character,
                    adjacent_part_numbers: vec![],
                });
            }
        }
    }
    // This is O(n²)!
    for part in parts.iter_mut() {
        part.is_adjacent = symbols.iter_mut().any(|symbol| {
            let ret = (part.x.start.saturating_sub(1)..part.x.end + 1)
                .contains(&symbol.x)
                && (part.y.saturating_sub(1)..=part.y + 1).contains(&symbol.y);
            if ret {
                symbol.adjacent_part_numbers.push(part.number);
            }
            ret
        });
    }
    println!(
        "Part 1 answer: {}",
        parts
            .iter()
            .map(|part| if part.is_adjacent { part.number } else { 0 })
            .sum::<u32>()
    );
    println!(
        "Part 2 answer: {}",
        symbols
            .iter()
            .map(|symbol| {
                if symbol.symbol == GEAR_SYMBOL
                    && symbol.adjacent_part_numbers.len() == 2
                {
                    symbol.adjacent_part_numbers.iter().product::<u32>()
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}
