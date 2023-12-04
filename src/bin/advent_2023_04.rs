struct Card {
    winning_numbers: Vec<u32>,
    present_numbers: Vec<u32>,
    num_matches: usize,
    num_copies: usize,
}

fn main() {
    let mut cards: Vec<Card> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, card) = line.split_once(": ").unwrap();
            let (winners, presents) = card.split_once(" | ").unwrap();
            let winning_numbers: Vec<u32> = winners
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();
            let present_numbers: Vec<u32> = presents
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();
            Card {
                num_matches: present_numbers
                    .iter()
                    .filter(|x| winning_numbers.contains(x))
                    .count(),
                winning_numbers,
                present_numbers,
                num_copies: 1,
            }
        })
        .collect();
    println!(
        "Puzzle 1 answer: {}",
        cards
            .iter()
            .map(|card| { 1 << card.num_matches >> 1 })
            .sum::<u32>()
    );
    for n in 0..cards.len() {
        let num_copies = cards[n].num_copies;
        for m in n + 1..=n + cards[n].num_matches {
            if m >= cards.len() {
                break;
            }
            cards[m].num_copies += num_copies;
        }
    }
    println!(
        "Puzzle 2 answer: {}",
        cards.iter().map(|card| { card.num_copies }).sum::<usize>()
    );
}
