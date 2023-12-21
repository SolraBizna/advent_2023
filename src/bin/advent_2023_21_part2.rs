use advent_2023::{Direction, Point, Tilemap};

const ELF_TARGET: u64 = 26501365;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Cell {
    Unknown,
    Rock,
    Reachable,
    Unreachable,
}

fn run_cells(prevmap: &Tilemap<Cell>, nextmap: &mut Tilemap<Cell>) {
    for y in 0..prevmap.get_height() {
        for x in 0..prevmap.get_width() {
            let center_cell = *prevmap.get_tile(Point { x, y }).unwrap();
            if center_cell == Cell::Rock {
                debug_assert_eq!(
                    *nextmap.get_tile(Point { x, y }).unwrap(),
                    Cell::Rock
                );
                continue;
            } else if center_cell != Cell::Unknown {
                nextmap.set_tile(Point { x, y }, center_cell);
            } else {
                // Unknown. Find me a neighbor
                for neighbor in Direction::ALL
                    .iter()
                    .copied()
                    .map(|dir| Point { x, y } + dir)
                {
                    match prevmap.get_tile(neighbor) {
                        Some(Cell::Reachable) => {
                            nextmap
                                .set_tile(Point { x, y }, Cell::Unreachable);
                            break;
                        }
                        Some(Cell::Unreachable) => {
                            nextmap.set_tile(Point { x, y }, Cell::Reachable);
                            break;
                        }
                        _ => {
                            debug_assert_eq!(
                                *nextmap.get_tile(Point { x, y }).unwrap(),
                                Cell::Unknown
                            );
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn get_populations(cellmap: &Tilemap<Cell>) -> [(u64, u64); 25] {
    let mut ret = [(0, 0); 25];
    for (y, row) in cellmap.rows().enumerate() {
        let y_sector = (y * 5) / cellmap.get_height() as usize;
        for (x, cell) in row.iter().copied().enumerate() {
            let x_sector = (x * 5) / cellmap.get_width() as usize;
            if cell == Cell::Reachable {
                let sector = x_sector + y_sector * 5;
                ret[sector].0 += 1;
            } else if cell == Cell::Unreachable {
                let sector = x_sector + y_sector * 5;
                ret[sector].1 += 1;
            }
        }
    }
    ret
}

fn count_plots(
    populations: &[[(u64, u64); 25]],
    cellmap: &Tilemap<Cell>,
    target: u64,
) -> u64 {
    let mut total = populations
        [target.min((populations.len() - 1) as u64) as usize]
        .iter()
        .map(|(a, _)| *a)
        .sum();
    // Add orthogonals
    for steppu in 1.. {
        let step = cellmap.get_width() as u64 * steppu;
        let Some(gen) = target.checked_sub(step) else {
            break;
        };
        let gen = gen.min((populations.len() - 1) as u64) as usize;
        for sector in [2, 10, 14, 22] {
            let pop = &populations[gen][sector];
            if *pop != (0, 0) && steppu == 4 {
                panic!("\x1b[1mORTHOGONAL {steppu}\x1b[0m")
            }
            if steppu % 2 == 0 {
                total += pop.0;
            } else {
                total += pop.1;
            }
        }
    }
    // Add orthodiagonals
    for steppu in 1.. {
        let step = cellmap.get_width() as u64 * steppu;
        let Some(gen) = target.checked_sub(step) else {
            break;
        };
        let gen = gen.min((populations.len() - 1) as u64) as usize;
        for sector in [1, 3, 5, 15, 9, 19, 21, 23] {
            let pop = &populations[gen][sector];
            if *pop != (0, 0) && steppu == 3 {
                panic!("\x1b[1mORTHODIAGONAL {steppu}\x1b[0m")
            }
            if steppu % 2 == 0 {
                total += pop.0;
            } else {
                total += pop.1;
            }
        }
    }
    // Add superdiagonals
    for steppu in 1.. {
        let step = cellmap.get_width() as u64 * steppu;
        let Some(gen) = target.checked_sub(step) else {
            break;
        };
        let multiplier = steppu + 1;
        let gen = gen.min((populations.len() - 1) as u64) as usize;
        for sector in [0, 4, 20, 24] {
            let pop = &populations[gen][sector];
            if *pop != (0, 0) && steppu == 2 {
                panic!("\x1b[1mSUPERDIAGONAL {steppu}\x1b[0m")
            }
            if steppu % 2 == 0 {
                total += pop.0 * multiplier;
            } else {
                total += pop.1 * multiplier;
            }
        }
    }
    total
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut cellmap = Tilemap::new_empty();
    let mut elf_point = None;
    for (y, line) in lines.enumerate() {
        buf.clear();
        buf.extend(line.char_indices().map(|(x, char)| {
            if char == 'S' {
                elf_point = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
                return Cell::Unknown;
            }
            match char {
                '.' => Cell::Unknown,
                '#' => Cell::Rock,
                _ => panic!(),
            }
        }));
        cellmap.add_row(&buf);
    }
    assert_eq!(cellmap.get_width() % 2, 1);
    assert_eq!(cellmap.get_height(), cellmap.get_width());
    let elf_point = elf_point.unwrap();
    assert_eq!(elf_point.x, cellmap.get_width() / 2);
    assert_eq!(elf_point.y, cellmap.get_height() / 2);
    let mut bigmap = Tilemap::new_empty();
    for _ in 0..5 {
        for row in cellmap.rows() {
            buf.clear();
            buf.extend(row);
            buf.extend(row);
            buf.extend(row);
            buf.extend(row);
            buf.extend(row);
            bigmap.add_row(&buf);
        }
    }
    bigmap.set_tile(
        Point {
            x: elf_point.x + cellmap.get_width() * 2,
            y: elf_point.y + cellmap.get_width() * 2,
        },
        Cell::Reachable,
    );
    let mut populations = vec![];
    populations.push(get_populations(&bigmap));
    let mut altmap = bigmap.clone();
    loop {
        run_cells(&bigmap, &mut altmap);
        populations.push(get_populations(&altmap));
        if bigmap == altmap {
            break;
        }
        std::mem::swap(&mut bigmap, &mut altmap);
    }
    // Stats time!
    let orthogonal_quadrants = [7, 11, 13, 17].map(|sector| {
        populations.iter().position(|x| x[sector].0 > 0).unwrap()
    });
    println!(
        "Wavefront reaches orthogonal quadrants in: {orthogonal_quadrants:?}"
    );
    let diagonal_quadrants = [6, 8, 16, 18].map(|sector| {
        populations.iter().position(|x| x[sector].0 > 0).unwrap()
    });
    println!(
        "Wavefront reaches diagonal quadrants in: {diagonal_quadrants:?}"
    );
    assert!(orthogonal_quadrants
        .iter()
        .all(|x| *x == ((cellmap.get_width() + 1) / 2) as usize));
    assert!(diagonal_quadrants
        .iter()
        .all(|x| *x == (cellmap.get_width() + 1) as usize));
    if true {
        drop(bigmap);
        let mut hugemap = Tilemap::new_empty();
        for _ in 0..9 {
            for row in cellmap.rows() {
                buf.clear();
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                buf.extend(row);
                hugemap.add_row(&buf);
            }
        }
        hugemap.set_tile(
            Point {
                x: elf_point.x + cellmap.get_width() * 4,
                y: elf_point.y + cellmap.get_width() * 4,
            },
            Cell::Reachable,
        );
        let mut altmap = hugemap.clone();
        let mut generation_count = 0;
        loop {
            println!(
                "{generation_count} {}",
                generation_count / (cellmap.get_width() as u64)
            );
            run_cells(&hugemap, &mut altmap);
            if hugemap == altmap {
                break;
            }
            generation_count += 1;
            assert_eq!(
                count_plots(&populations, &cellmap, generation_count),
                altmap.iter().filter(|x| **x == Cell::Reachable).count()
                    as u64
            );
            std::mem::swap(&mut hugemap, &mut altmap);
        }
    }
    println!(
        "And the part 2 puzzle output is: {}",
        count_plots(&populations, &cellmap, ELF_TARGET)
    );
}
