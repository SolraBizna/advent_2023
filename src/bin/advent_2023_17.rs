use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
};

use advent_2023::{Direction, Point, Tilemap};

const MAX_STRAIGHTS: u8 = 3;

#[derive(Debug, Copy, Clone)]
struct Cost {
    heatloss: u32,
    dir: Option<Direction>,
    dir_count: u8,
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.heatloss == other.heatloss
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.heatloss.cmp(&other.heatloss).reverse() {
            Ordering::Equal => self.dir_count.cmp(&other.dir_count).reverse(),
            x => x,
        }
    }
}

impl Eq for Cost {}

impl Display for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self.dir {
                None => '?',
                Some(Direction::North) => '↑',
                Some(Direction::South) => '↓',
                Some(Direction::East) => '→',
                Some(Direction::West) => '←',
            }
        )
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut pricemap = Tilemap::new_empty();
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(|x| x.to_digit(10).unwrap()));
        pricemap.add_row(&buf);
    }
    let mut costmap: Tilemap<Cost> = Tilemap::new_with(
        Cost {
            heatloss: u32::MAX,
            dir: None,
            dir_count: 0,
        },
        pricemap.get_width(),
        pricemap.get_height(),
    );
    costmap.set_tile(
        Point { x: 0, y: 0 },
        Cost {
            heatloss: 0,
            dir: None,
            dir_count: 0,
        },
    );
    // DIJKSTRAAAAAA (formerly)
    let mut live_nodes: BinaryHeap<(Cost, Point)> = vec![(
        Cost {
            heatloss: 0,
            dir: None,
            dir_count: 0,
        },
        Point { x: 0, y: 0 },
    )]
    .into();
    let mut dead_nodes: HashSet<(Direction, Point)> = HashSet::new();
    while let Some((cost_here, point_here)) = live_nodes.pop() {
        if costmap
            .get_tile(Point {
                x: costmap.get_width() - 1,
                y: costmap.get_height() - 1,
            })
            .unwrap()
            .heatloss
            < u32::MAX
        {
            break;
        }
        for dir in Direction::ALL {
            if Some(-*dir) == cost_here.dir
                || (Some(*dir) == cost_here.dir
                    && cost_here.dir_count >= MAX_STRAIGHTS)
            {
                continue;
            }
            let dest = point_here + *dir;
            let Some(&price_of_dest) = pricemap.get_tile(dest) else {
                continue;
            };
            let cost_of_dest_ref = costmap.get_tile_mut(dest).unwrap();
            let heatloss_of_dest = price_of_dest + cost_here.heatloss;
            let cost_of_dest = Cost {
                heatloss: heatloss_of_dest,
                dir: Some(*dir),
                dir_count: if Some(*dir) == cost_here.dir {
                    cost_here.dir_count + 1
                } else {
                    1
                },
            };
            if heatloss_of_dest < cost_of_dest_ref.heatloss {
                assert_eq!(
                    cost_of_dest_ref.heatloss,
                    u32::MAX,
                    "Meiko was right! Dijkstra didn't hold!"
                );
                *cost_of_dest_ref = cost_of_dest;
            } else {
                // not a shorter path, Dijkstra was right!
            }
            let flapjack = (*dir, point_here);
            if !dead_nodes.contains(&flapjack) {
                live_nodes.push((cost_of_dest, dest));
                dead_nodes.insert(flapjack);
            }
        }
    }
    println!(
        "Puzzle 1 answer: {}",
        costmap
            .get_tile(Point {
                x: costmap.get_width() - 1,
                y: costmap.get_height() - 1,
            })
            .unwrap()
            .heatloss
    );
}
