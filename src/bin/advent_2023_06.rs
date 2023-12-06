#[derive(Debug)]
struct Race {
    /// Number of milliseconds the race will last
    time: u64,
    /// Number of millimeters the recordholder went
    distance: u64,
}

impl Race {
    fn get_num_solutions(&self) -> u64 {
        (0..self.time)
            .map(|charge_time| (self.time - charge_time) * charge_time)
            .filter(|distance| *distance > self.distance)
            .count() as u64
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let line = lines.next().unwrap();
    let (_, rest) = line.split_once(':').unwrap();
    let times = rest
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap());
    let line = lines.next().unwrap();
    let (_, rest) = line.split_once(':').unwrap();
    let distances = rest
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap());
    let races: Vec<Race> = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    println!(
        "Part 1 solution: {}",
        races.iter().map(Race::get_num_solutions).product::<u64>(),
    );
    let superrace = races.iter().fold(
        Race {
            time: 0,
            distance: 0,
        },
        |total, race| Race {
            time: total.time * 10u64.pow(race.time.ilog10() + 1) + race.time,
            distance: total.distance * 10u64.pow(race.distance.ilog10() + 1)
                + race.distance,
        },
    );
    println!("Part 2 solution: {}", superrace.get_num_solutions());
}
