use advent_2023::{Direction, Point, Tilemap};

fn display_map(map: &Tilemap<bool>) {
    for row in map.rows() {
        for el in row.iter() {
            print!("{}", if *el { '◘' } else { '•' });
        }
        println!();
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut rockmap = Tilemap::new_empty();
    let mut elf_point = None;
    for (y, line) in lines.enumerate() {
        buf.clear();
        buf.extend(line.char_indices().map(|(x, char)| {
            if char == 'S' {
                elf_point = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
            char == '#'
        }));
        rockmap.add_row(&buf);
    }
    let elf_point = elf_point.unwrap();
    let mut active_steps = Vec::with_capacity(
        (rockmap.get_width() * rockmap.get_height()) as usize,
    );
    let mut next_steps = Vec::with_capacity(
        (rockmap.get_width() * rockmap.get_height()) as usize,
    );
    active_steps.push(elf_point);
    let mut reachmap =
        Tilemap::new_with(false, rockmap.get_width(), rockmap.get_height());
    reachmap.set_tile(elf_point, true);
    for step in 1..=64 {
        assert!(next_steps.is_empty());
        for start_point in active_steps.drain(..) {
            for next_step in
                Direction::ALL.iter().copied().map(|dir| start_point + dir)
            {
                let Some(false) = rockmap.get_tile(next_step) else {
                    continue;
                };
                if step % 2 == 0 {
                    let Some(false) = reachmap.get_tile(next_step) else {
                        continue;
                    };
                    reachmap.set_tile(next_step, true);
                } else if !Direction::ALL
                    .iter()
                    .copied()
                    .map(|dir| next_step + dir)
                    .any(|つぎのつぎの| {
                        Some(&false) == rockmap.get_tile(つぎのつぎの)
                            && Some(&false) == reachmap.get_tile(つぎのつぎの)
                    })
                {
                    continue;
                }
                next_steps.push(next_step);
            }
        }
        std::mem::swap(&mut active_steps, &mut next_steps);
        next_steps.clear();
    }
    display_map(&reachmap);
    println!(
        "Puzzle 1 answer: {}",
        reachmap.iter().filter(|x| **x).count()
    );
}
