fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lines()
            .map(|x| {
                let x = x.unwrap();
                let digits = x.chars().filter(|x| x.is_ascii_digit());
                let first = digits.clone().next().unwrap();
                let last = digits.last().unwrap();
                [first, last]
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
    );
}
