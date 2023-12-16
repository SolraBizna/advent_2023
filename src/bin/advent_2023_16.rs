use std::collections::HashSet;

use advent_2023::{Direction, Point, Tilemap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Nothing,
    Foreslash,
    Backslash,
    Bar,
    Dash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Laser {
    beam_dir: Direction,
    pos: Point,
}

impl Mirror {
    pub fn from_char(ch: char) -> Mirror {
        match ch {
            '.' => Mirror::Nothing,
            '/' => Mirror::Foreslash,
            '\\' => Mirror::Backslash,
            '|' => Mirror::Bar,
            '-' => Mirror::Dash,
            _ => panic!("Corrupted puzzle"),
        }
    }
    pub fn bounce(
        &self,
        incoming_laser_dir: Direction,
    ) -> &'static [Direction] {
        match self {
            Mirror::Nothing => match incoming_laser_dir {
                Direction::North => &[Direction::North],
                Direction::South => &[Direction::South],
                Direction::East => &[Direction::East],
                Direction::West => &[Direction::West],
            },
            Mirror::Foreslash => match incoming_laser_dir {
                Direction::North => &[Direction::East],
                Direction::South => &[Direction::West],
                Direction::East => &[Direction::North],
                Direction::West => &[Direction::South],
            },
            Mirror::Backslash => match incoming_laser_dir {
                Direction::North => &[Direction::West],
                Direction::South => &[Direction::East],
                Direction::East => &[Direction::South],
                Direction::West => &[Direction::North],
            },
            Mirror::Bar => match incoming_laser_dir {
                Direction::North => &[Direction::North],
                Direction::South => &[Direction::South],
                Direction::East => &[Direction::North, Direction::South],
                Direction::West => &[Direction::North, Direction::South],
            },
            Mirror::Dash => match incoming_laser_dir {
                Direction::North => &[Direction::East, Direction::West],
                Direction::South => &[Direction::East, Direction::West],
                Direction::East => &[Direction::East],
                Direction::West => &[Direction::West],
            },
        }
    }
}

fn energize(
    tilemap: &Tilemap<Mirror>,
    obrien_dir: Direction,
    obrien_pos: Point,
) -> usize {
    let mut heatmap: Tilemap<bool> =
        Tilemap::new(tilemap.get_width(), tilemap.get_height());
    let mut lasers = vec![Laser {
        beam_dir: obrien_dir,
        pos: obrien_pos,
    }];
    // definitely large enough
    let mut ever_laser = HashSet::with_capacity(
        (tilemap.get_width() * tilemap.get_height() * 4) as usize,
    );
    while !lasers.is_empty() {
        let mut new_lasers = vec![];
        for in_laser in lasers.iter() {
            if ever_laser.contains(in_laser) {
                continue;
            } else {
                ever_laser.insert(*in_laser);
            }
            if let Some(mirror) = tilemap.get_tile(in_laser.pos) {
                heatmap.set_tile(in_laser.pos, true);
                for direction in mirror.bounce(in_laser.beam_dir) {
                    new_lasers.push(Laser {
                        beam_dir: *direction,
                        pos: in_laser.pos + *direction,
                    })
                }
            }
        }
        lasers.clear();
        std::mem::swap(&mut lasers, &mut new_lasers);
    }
    heatmap.iter().filter(|x| **x).count()
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut tilemap = Tilemap::new_empty();
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(Mirror::from_char));
        tilemap.add_row(&buf);
    }
    println!(
        "Puzzle 1 solution: {}",
        energize(&tilemap, Direction::East, Point { x: 0, y: 0 })
    );
    let north_iter = (0..tilemap.get_width()).map(|x| {
        (
            Direction::North,
            Point {
                x,
                y: tilemap.get_height() - 1,
            },
        )
    });
    let south_iter = (0..tilemap.get_width())
        .map(|x| (Direction::South, Point { x, y: 0 }));
    let east_iter = (0..tilemap.get_height())
        .map(|y| (Direction::East, Point { x: 0, y }));
    let west_iter = (0..tilemap.get_height()).map(|y| {
        (
            Direction::East,
            Point {
                x: tilemap.get_width() - 1,
                y,
            },
        )
    });
    println!(
        "Puzzle 2 solution: {}",
        north_iter
            .chain(south_iter.chain(east_iter.chain(west_iter)))
            .map(|(start_dir, start_pos)| {
                energize(&tilemap, start_dir, start_pos)
            })
            .max()
            .unwrap_or(0)
    )
}
