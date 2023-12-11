use std::collections::HashSet;

use advent_2023::{Point, Tilemap};

fn find_expanded_path_distance(
    galaxies: &[Point],
    blank_rows: &HashSet<i32>,
    blank_columns: &HashSet<i32>,
    expansion_factor: i32,
) -> i64 {
    galaxies
        .iter()
        .map(|a| {
            galaxies
                .iter()
                .map(|b| {
                    // This is harmless if done on the same galaxy in both a and b,
                    // because the distance would still be zero.
                    let d = *a - *b;
                    let mut ret = (d.x.abs() + d.y.abs()) as i64;
                    for x in a.x.min(b.x) + 1..a.x.max(b.x) {
                        if blank_columns.contains(&x) {
                            ret += expansion_factor as i64;
                        }
                    }
                    for y in a.y.min(b.y) + 1..a.y.max(b.y) {
                        if blank_rows.contains(&y) {
                            ret += expansion_factor as i64;
                        }
                    }
                    ret
                })
                .sum::<i64>()
        })
        .sum::<i64>()
        / 2 // ...
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut galaxymap = Tilemap::new_empty();
    let mut buf: Vec<bool> = vec![];
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(|x| x == '#'));
        galaxymap.add_row(&buf);
    }
    // Find all galaxy-less rows
    let blank_rows: HashSet<i32> = (0..galaxymap.get_height())
        .filter(|y| galaxymap.get_row(*y).unwrap().iter().all(|x| !x))
        .collect();
    // Find all galaxy-less columns
    let blank_columns: HashSet<i32> = (0..galaxymap.get_width())
        .filter(|x| {
            (0..galaxymap.get_height())
                .map(|y| *galaxymap.get_tile(Point { x: *x, y }).unwrap())
                .all(|x| !x)
        })
        .collect();
    // Find every galaxy
    let galaxies: Vec<Point> = galaxymap.find_tiles(|x| *x).collect();
    println!(
        "Puzzle 1 answer: {}",
        find_expanded_path_distance(&galaxies, &blank_rows, &blank_columns, 1)
    );
    println!(
        "Puzzle 2 answer: {}",
        find_expanded_path_distance(
            &galaxies,
            &blank_rows,
            &blank_columns,
            999_999
        )
    );
}
