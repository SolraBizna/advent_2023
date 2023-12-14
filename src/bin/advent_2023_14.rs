use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

use advent_2023::{Point, Tilemap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Blank,
    Movable,
    Immovable,
}

impl Tile {
    fn from_char(ch: char) -> Tile {
        match ch {
            'O' => Tile::Movable,
            '#' => Tile::Immovable,
            '.' => Tile::Blank,
            _ => panic!("corrupted puzzle!"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Tile::Blank => '·',
                Tile::Movable => 'O',
                Tile::Immovable => '#',
            }
        )
    }
}

fn roll_boulder(
    tilemap: &mut Tilemap<Tile>,
    mut position: Point,
    direction: Point,
) -> bool {
    let mut changed = false;
    while let Some(Tile::Movable) = tilemap.get_tile(position) {
        let destination = position + direction;
        let Some(Tile::Blank) = tilemap.get_tile(destination) else {
            break;
        };
        tilemap.set_tile(position, Tile::Blank);
        tilemap.set_tile(destination, Tile::Movable);
        changed = true;
        position = destination;
    }
    changed
}

fn roll_boulders(
    mut tilemap: Tilemap<Tile>,
    direction: Point,
) -> Tilemap<Tile> {
    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..tilemap.get_height() {
            for x in 0..tilemap.get_width() {
                changed =
                    roll_boulder(&mut tilemap, Point { x, y }, direction)
                        || changed;
            }
        }
    }
    tilemap
}

fn spin_cycle(mut tilemap: Tilemap<Tile>) -> Tilemap<Tile> {
    tilemap = roll_boulders(tilemap, Point { x: 0, y: -1 });
    tilemap = roll_boulders(tilemap, Point { x: -1, y: 0 });
    tilemap = roll_boulders(tilemap, Point { x: 0, y: 1 });
    tilemap = roll_boulders(tilemap, Point { x: 1, y: 0 });
    tilemap
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut tilemap = Tilemap::new_empty();
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(Tile::from_char));
        tilemap.add_row(&buf);
    }
    println!(
        "Puzzle 1 answer: {}",
        roll_boulders(tilemap.clone(), Point { x: 0, y: -1 })
            .rows()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|x| **x == Tile::Movable).count()
                    * (tilemap.get_height() - y as i32) as usize
            })
            .sum::<usize>()
    );
    // this is as close as I'm ever going to get to Hashlife
    let mut cur_tilemap = tilemap;
    let mut previous_generations = HashMap::with_capacity(16384);
    previous_generations.insert(cur_tilemap.clone(), 0);
    const NUM_GENERATIONS: u32 = 1_000_000_000;
    for generation in 1..=NUM_GENERATIONS {
        cur_tilemap = spin_cycle(cur_tilemap);
        println!("Generation {generation}:\n{cur_tilemap}");
        if let Some(cycle_start_generation) =
            previous_generations.get(&cur_tilemap)
        {
            println!("Found cycle! {cycle_start_generation} .. {generation}");
            let num_aheads = (NUM_GENERATIONS - cycle_start_generation)
                % (generation - cycle_start_generation);
            dbg!(num_aheads);
            for _ in 0..num_aheads {
                cur_tilemap = spin_cycle(cur_tilemap);
            }
            break;
        }
        previous_generations.insert(cur_tilemap.clone(), generation);
    }
    println!(
        "Puzzle 2 answer: {}",
        cur_tilemap
            .rows()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|x| **x == Tile::Movable).count()
                    * (cur_tilemap.get_height() - y as i32) as usize
            })
            .sum::<usize>()
    );
}
