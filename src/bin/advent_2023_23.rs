use std::sync::atomic::{AtomicU32, Ordering};

use advent_2023::{Direction, Point, Tilemap};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    EastSlope,
    SouthSlope,
}

impl Tile {
    fn from_char(ch: char) -> Tile {
        match ch {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            '>' => Tile::EastSlope,
            'v' => Tile::SouthSlope,
            _ => panic!(),
        }
    }
    fn can_enter_along(&self, direction: Direction) -> bool {
        match (self, direction) {
            (Tile::Wall, _) => false,
            (Tile::Floor, _) => true,
            (Tile::SouthSlope, Direction::South) => true,
            (Tile::EastSlope, Direction::East) => true,
            _ => false,
        }
    }
}

static LONGEST: AtomicU32 = AtomicU32::new(0);

fn find_path_from(
    tile_map: &Tilemap<Tile>,
    mut been_map: Tilemap<bool>,
    mut position: Point,
    mut current_length: u32,
) -> Option<u32> {
    loop {
        if position.y == tile_map.get_height() - 1 {
            let prev = LONGEST.fetch_max(current_length, Ordering::Relaxed);
            if prev < current_length {
                eprint!("Longest: {}  \r", current_length);
            }
            return Some(current_length);
        }
        let mut possible_destinations =
            Direction::ALL.iter().copied().filter_map(|direction| {
                let destination = position + direction;
                let Some(false) = been_map.get_tile(destination) else {
                    return None;
                };
                if let Some(dest_tile) = tile_map.get_tile(destination) {
                    if dest_tile.can_enter_along(direction) {
                        return Some(destination);
                    }
                }
                None
            });
        match possible_destinations.clone().count() {
            0 => return None,
            1 => {
                position = possible_destinations.next().unwrap();
                been_map.set_tile(position, true);
                current_length += 1;
            }
            _ => {
                return possible_destinations
                    .filter_map(|destination| {
                        let mut been_map = been_map.clone();
                        been_map.set_tile(destination, true);
                        find_path_from(
                            tile_map,
                            been_map,
                            destination,
                            current_length + 1,
                        )
                    })
                    .max();
            }
        }
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut tile_map = Tilemap::new_empty();
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(Tile::from_char));
        tile_map.add_row(&buf);
    }
    let start_point = Point { x: 1, y: 0 };
    let end_point = Point {
        x: tile_map.get_width() - 2,
        y: tile_map.get_width() - 1,
    };
    assert!(tile_map.get_tile(start_point) == Some(&Tile::Floor));
    assert!(tile_map.get_tile(end_point) == Some(&Tile::Floor));
    let mut been_map =
        Tilemap::new(tile_map.get_width(), tile_map.get_height());
    been_map.set_tile(start_point, true);
    let longest_path =
        find_path_from(&tile_map, been_map.clone(), start_point, 0).unwrap();
    println!("Part 1 answer: {}", longest_path);
    // Now, it turns out the slopes are easy, so try again!
    for tile in tile_map.iter_mut() {
        if *tile == Tile::SouthSlope || *tile == Tile::EastSlope {
            *tile = Tile::Floor;
        }
    }
    let longest_path =
        find_path_from(&tile_map, been_map, start_point, 0).unwrap();
    println!("Part 2 answer: {}", longest_path);
}
