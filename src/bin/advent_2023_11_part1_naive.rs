use advent_2023::{Point, Tilemap};

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut galaxymap = Tilemap::new_empty();
    let mut buf: Vec<bool> = vec![];
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(|x| x == '#'));
        galaxymap.add_row(&buf);
    }
    // Expand all galaxy-less rows
    for y in (0..galaxymap.get_height()).rev() {
        if galaxymap.get_row(y).unwrap().iter().all(|x| !x) {
            galaxymap.insert_blank_row(y);
        }
    }
    // Expand all galaxy-less columns
    for x in (0..galaxymap.get_width()).rev() {
        if (0..galaxymap.get_height())
            .map(|y| *galaxymap.get_tile(Point { x, y }).unwrap())
            .all(|x| !x)
        {
            galaxymap.insert_blank_column(x);
        }
    }
    // Find every galaxy
    let galaxies: Vec<Point> = galaxymap.find_tiles(|x| *x).collect();
    // Total the Manhattan distance between them
    let total_distance: i32 = galaxies
        .iter()
        .map(|a| {
            galaxies
                .iter()
                .map(|b| {
                    // This is harmless if done on the same galaxy in both a and b,
                    // because the distance would still be zero.
                    let d = *a - *b;
                    d.x.abs() + d.y.abs()
                })
                .sum::<i32>()
        })
        .sum::<i32>()
        / 2; // ...
    println!("Puzzle 1 answer: {total_distance}");
}
