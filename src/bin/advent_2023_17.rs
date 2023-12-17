use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    rc::Rc,
};

use advent_2023::{Direction, Point, Tilemap};

#[derive(Debug, Clone)]
struct PathNode {
    parent: Option<Rc<PathNode>>,
    pos: Point,
    heatloss: u32,
    entry_dir: Option<Direction>,
    dir_count: u8,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.heatloss == other.heatloss
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heatloss.cmp(&other.heatloss).reverse()
    }
}

impl Eq for PathNode {}

#[allow(unused)]
fn display_path(end_node: &PathNode, width: i32, height: i32) {
    let mut tiles = Tilemap::new_with('·', width, height);
    let mut next_node = Some(end_node);
    while let Some(node) = next_node {
        tiles.set_tile(
            node.pos,
            node.entry_dir.map(|x| x.to_arrow()).unwrap_or('?'),
        );
        next_node = node.parent.as_ref().map(|x| x.as_ref());
    }
    print!("{}", tiles);
}

fn find_path(
    pricemap: &Tilemap<u8>,
    min_straights: u8,
    max_straights: u8,
) -> PathNode {
    let mut live_nodes: BinaryHeap<PathNode> = vec![PathNode {
        parent: None,
        pos: Point { x: 0, y: 0 },
        heatloss: 0,
        entry_dir: None,
        dir_count: 0,
    }]
    .into();
    let mut dead_nodes: HashSet<(Point, Direction, u8)> =
        HashSet::with_capacity(1048576);
    while let Some(path_node) = live_nodes.pop() {
        if path_node.pos
            == (Point {
                x: pricemap.get_width() - 1,
                y: pricemap.get_height() - 1,
            })
        {
            return path_node;
        }
        let 死のノード = path_node
            .entry_dir
            .map(|entry_dir| (path_node.pos, entry_dir, path_node.dir_count));
        if let Some(死のノード) = 死のノード {
            if dead_nodes.contains(&死のノード) {
                continue;
            }
            dead_nodes.insert(死のノード);
        }
        let path_node = Rc::new(path_node);
        for dir in Direction::ALL {
            let dir = *dir;
            if Some(-dir) == path_node.entry_dir
                || (Some(dir) == path_node.entry_dir
                    && path_node.dir_count >= max_straights)
                || (path_node.entry_dir.is_some()
                    && Some(dir) != path_node.entry_dir
                    && path_node.dir_count < min_straights)
            {
                // no going too far forwards
                continue;
            }
            let dest_pos = path_node.pos + dir;
            let Some(&price) = pricemap.get_tile(dest_pos) else {
                continue;
            };
            let next_node = PathNode {
                parent: Some(path_node.clone()),
                pos: dest_pos,
                heatloss: path_node.heatloss + price as u32,
                entry_dir: Some(dir),
                dir_count: if path_node.entry_dir == Some(dir) {
                    path_node.dir_count + 1
                } else {
                    1
                },
            };
            live_nodes.push(next_node);
        }
    }
    unreachable!()
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = vec![];
    let mut pricemap = Tilemap::new_empty();
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(|x| x.to_digit(10).unwrap() as u8));
        pricemap.add_row(&buf);
    }
    println!("Puzzle 1 answer: {}", find_path(&pricemap, 1, 3).heatloss);
    println!("Puzzle 2 answer: {}", find_path(&pricemap, 4, 10).heatloss);
}
