use std::cmp::Ordering;

use advent_2023::{Direction, Point, Tilemap};

#[derive(Debug)]
struct Command {
    dir: Direction,
    count: i32,
    color: u32,
}

#[derive(Debug)]
struct Command2 {
    dir: Direction,
    count: i32,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

const NO_PIXEL: u32 = u32::MAX;
const FILL_PIXEL: u32 = 0xFF00FFu32;

fn display_map(tilemap: &Tilemap<u32>, dirmap: &Tilemap<Option<Direction>>) {
    for (color_rows, dir_rows) in tilemap.rows().zip(dirmap.rows()) {
        for (pix, dir) in color_rows.iter().copied().zip(dir_rows.iter()) {
            if pix == NO_PIXEL {
                print!("\x1B[0m·");
            } else {
                let r = (pix >> 16) & 255;
                let g = (pix >> 8) & 255;
                let b = pix & 255;
                print!(
                    "\x1B[48;2;{r};{g};{b}m{}",
                    dir.map(|x| x.to_arrow()).unwrap_or('?')
                );
            }
        }
        println!("\x1B[0m");
    }
}

#[derive(Debug)]
struct Crossing {
    is_upward: bool,
    is_downward: bool,
    x_coord: i32,
}
fn part2_method(
    line_segments: &[LineSegment],
    min_x: i32,
    width: usize,
) -> u64 {
    let mut crossings: Vec<Crossing> = Vec::with_capacity(15);
    let mut y_coordinates: Vec<i32> = line_segments
        .iter()
        .filter(|segment| segment.start.y != segment.end.y)
        .flat_map(|segment| {
            [
                segment.start.y.min(segment.end.y) - 1,
                segment.start.y.min(segment.end.y),
                segment.start.y.min(segment.end.y) + 1,
                segment.start.y.max(segment.end.y) - 1,
                segment.start.y.max(segment.end.y),
                segment.start.y.max(segment.end.y) + 1,
            ]
            .into_iter()
        })
        .collect();
    y_coordinates.sort();
    // remove duplicates...
    for i in (0..y_coordinates.len() - 1).rev() {
        if y_coordinates[i] == y_coordinates[i + 1] {
            y_coordinates.remove(i + 1);
        }
    }
    // remove the out-of-bounds ones at the start and end
    y_coordinates.pop();
    y_coordinates.remove(0);
    let mut previous_y = y_coordinates[0];
    let mut previous_area = 0;
    let mut total = 0;
    let mut buf = vec![false; width];
    for y in y_coordinates.iter().copied() {
        total += previous_area * (y - previous_y) as u64;
        previous_y = y;
        crossings.clear();
        crossings.extend(
            line_segments
                .iter()
                .filter(|segment| segment.start.y != segment.end.y)
                .filter(|segment| {
                    (segment.start.y.min(segment.end.y)
                        ..=segment.start.y.max(segment.end.y))
                        .contains(&y)
                })
                .map(|segment| Crossing {
                    is_upward: segment.start.y > segment.end.y,
                    is_downward: segment.start.y < segment.end.y,
                    x_coord: segment.start.x,
                }),
        );
        crossings.sort_by(|a, b| a.x_coord.cmp(&b.x_coord));
        assert!(crossings[0].is_upward); // assert clockwise winding
        for i in (0..crossings.len() - 1).rev() {
            // kill all but leftmost of consecutive ups
            if crossings[i].is_upward && crossings[i + 1].is_upward {
                crossings.remove(i + 1);
            }
            // kill all but rightmost of consecutive downs
            if crossings[i].is_downward && crossings[i + 1].is_downward {
                crossings.remove(i);
            }
        }
        if cfg!(debug_assertions) {
            for q in crossings.chunks(2) {
                assert_eq!(q.len(), 2);
                assert!(q[0].is_upward);
                assert!(q[1].is_downward);
            }
        }
        buf[..].fill(false);
        for pair in crossings.chunks_exact(2) {
            buf[(pair[0].x_coord - min_x) as usize
                ..=(pair[1].x_coord - min_x) as usize]
                .fill(true);
        }
        for segment in line_segments.iter().filter(|segment| {
            segment.start.y == segment.end.y && segment.start.y == y
        }) {
            buf[(segment.start.x.min(segment.end.x) - min_x) as usize
                ..=(segment.start.x.max(segment.end.x) - min_x) as usize]
                .fill(true);
        }
        previous_area = buf.iter().filter(|x| **x).count() as u64;
    }
    assert_eq!(previous_y, *y_coordinates.last().unwrap());
    total += previous_area;
    total
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let commands: Vec<Command> = lines
        .map(|line| {
            let splat: Vec<&str> = line.split(' ').collect();
            let &[dir, count, color] = &splat[..] else {
                panic!()
            };
            let dir = match dir {
                "U" => Direction::North,
                "D" => Direction::South,
                "R" => Direction::East,
                "L" => Direction::West,
                x => panic!("Unknown direction {x:?}!"),
            };
            let count = count.parse().unwrap();
            let color =
                color.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();
            let color = u32::from_str_radix(color, 16).unwrap();
            Command { dir, count, color }
        })
        .collect();
    // Find extents
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut pos = Point { x: 0, y: 0 };
    for command in commands.iter() {
        pos = pos + command.dir * command.count;
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }
    println!("X ranges from {min_x}..={max_x}");
    println!("Y ranges from {min_y}..={max_y}");
    let start_x = -min_x;
    let start_y = -min_y;
    let width = (max_x - min_x) + 1;
    let height = (max_y - min_y) + 1;
    println!("Map is {width}x{height}, start at {start_x},{start_y}");
    let mut colormap = Tilemap::new_with(NO_PIXEL, width, height);
    let mut dirmap = Tilemap::new_with(None, width, height);
    let mut pos = Point {
        x: start_x,
        y: start_y,
    };
    for command in commands.iter() {
        for _ in 0..command.count {
            pos = pos + command.dir;
            colormap.set_tile(pos, command.color);
            dirmap.set_tile(pos, Some(command.dir));
        }
    }
    println!("Lines:");
    display_map(&colormap, &dirmap);
    // Fill!
    for _ in 0..4 {
        for (color_row, dir_row) in colormap.rows_mut().zip(dirmap.rows()) {
            let mut fill = false;
            for (pixel, dir) in color_row.iter_mut().zip(dir_row.iter()) {
                if *pixel == NO_PIXEL {
                    if fill {
                        *pixel = FILL_PIXEL;
                    }
                    continue;
                } else if *pixel == FILL_PIXEL {
                    fill = true;
                } else {
                    match dir {
                        Some(Direction::North) => fill = true,
                        Some(_) => fill = false,
                        _ => (),
                    }
                }
            }
        }
        colormap = colormap.rotate_cw();
        dirmap = dirmap.rotate_cw();
        for dir in dirmap.iter_mut() {
            *dir = dir.map(|x| x.rotate_cw());
        }
    }
    println!("Fill:");
    display_map(&colormap, &dirmap);
    println!(
        "Okay, here we go. Puzzle 1: {}",
        colormap.iter().filter(|x| **x != NO_PIXEL).count()
    );
    // Test part 2 calculation on part 1...
    let mut pos = Point { x: 0, y: 0 };
    let line_segments: Vec<LineSegment> = commands
        .iter()
        .map(|command| {
            let start = pos;
            pos = pos + command.dir * command.count;
            let end = pos;
            LineSegment { start, end }
        })
        .collect();
    println!(
        "Part 1 answer, part 2 method: {}",
        part2_method(&line_segments, min_x, width as usize)
    );
    // The REAL puzzle!
    let commands: Vec<Command2> = commands
        .iter()
        .map(|command| {
            let dir = match (command.color & 15) {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => unreachable!(),
            };
            let count = (command.color >> 4) as i32;
            Command2 { dir, count }
        })
        .collect();
    // Reduce, reuse, re---HEY!
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut pos = Point { x: 0, y: 0 };
    for command in commands.iter() {
        pos = pos + command.dir * command.count;
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }
    println!("X ranges from {min_x}..={max_x}");
    println!("Y ranges from {min_y}..={max_y}");
    let start_x = -min_x;
    let start_y = -min_y;
    let width = (max_x - min_x) + 1;
    let height = (max_y - min_y) + 1;
    println!(
        "Map would be {width}x{height}, would start at {start_x},{start_y}"
    );
    let mut pos = Point { x: 0, y: 0 };
    let line_segments: Vec<LineSegment> = commands
        .iter()
        .map(|command| {
            let start = pos;
            pos = pos + command.dir * command.count;
            let end = pos;
            LineSegment { start, end }
        })
        .collect();
    println!(
        "Part 2 answer: {}",
        part2_method(&line_segments, min_x, width as usize)
    );
}
