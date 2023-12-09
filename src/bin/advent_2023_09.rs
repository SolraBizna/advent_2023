fn predict(sequence: &[i32]) -> i32 {
    let delta: Vec<i32> = sequence
        .windows(2)
        .map(|x| {
            let &[prev, next] = x else { panic!() };
            next - prev
        })
        .collect();
    if delta.iter().all(|x| *x == 0) {
        0
    } else {
        delta.last().unwrap() + predict(&delta)
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let values: Vec<Vec<i32>> = lines
        .map(|x| x.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect();
    println!(
        "Part 1 answer: {}",
        values
            .iter()
            .map(|x| x.last().unwrap() + predict(x))
            .sum::<i32>(),
    );
    // gross
    let seulav: Vec<Vec<i32>> = values
        .iter()
        .map(|x| {
            let mut x = x.clone();
            x.reverse();
            x
        })
        .collect();
    println!(
        "Part 2 answer: {}",
        seulav
            .iter()
            .map(|x| x.last().unwrap() + predict(x))
            .sum::<i32>(),
    );
}
