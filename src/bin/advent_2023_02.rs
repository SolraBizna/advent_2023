use std::collections::HashMap;

type Round = HashMap<String, u32>;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn main() {
    let games: Vec<Game> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (before, after) = line.split_once(": ").unwrap();
            let before = before.strip_prefix("Game ").unwrap();
            let id = before.parse().unwrap();
            let rounds = after
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        .map(|color| {
                            let (amount, color) = color.split_once(' ').unwrap();
                            let amount = amount.parse().unwrap();
                            (color.to_string(), amount)
                        })
                        .collect()
                })
                .collect();
            Game { id, rounds }
        })
        .collect();
    println!(
        "Part 1 answer: {}",
        games
            .iter()
            .map(|game| {
                if game.rounds.iter().all(|round| {
                    round.get("red").copied().unwrap_or(0) <= 12
                        && round.get("green").copied().unwrap_or(0) <= 13
                        && round.get("blue").copied().unwrap_or(0) <= 14
                }) {
                    game.id
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
    println!(
        "Part 2 answer: {}",
        games
            .iter()
            .map(|game| {
                game.rounds
                    .iter()
                    .fold(HashMap::new(), |mut least, round| {
                        for (color, amount) in round.iter() {
                            let entry = least.entry(color.clone()).or_insert(0);
                            *entry = (*entry).max(*amount);
                        }
                        least
                    })
                    .into_values()
                    .product::<u32>()
            })
            .sum::<u32>()
    );
}
