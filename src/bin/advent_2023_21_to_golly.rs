// I tried to chicken out and use golly for this, but it was too big for golly.

use advent_2023::{Point, Tilemap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Cell {
    Unknown,
    Rock,
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
    //println!("Elf point: {elf_point:?}");
    println!(
        "x = {}, y = {}, rule = Advent-2023-21",
        cellmap.get_width(),
        cellmap.get_height()
    );
    for (y, row) in cellmap.rows().enumerate() {
        for c in row.iter().copied() {
            print!(
                "{}",
                match c {
                    Cell::Unknown => '.',
                    Cell::Rock => 'A',
                }
            );
        }
        if y as i32 == cellmap.get_height() - 1 {
            print!("!");
        } else {
            print!("$");
        }
    }
}
