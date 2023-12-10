use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Neg, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Pipe {
    #[default]
    None,
    Horizontal,
    Vertical,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Snarbolax,
}

const ALL_CONNECTIONS: &[Point] = &[
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
];

impl Pipe {
    fn get_connections(&self) -> &'static [Point] {
        match self {
            Pipe::None => &[],
            Pipe::Horizontal => &[Point { x: -1, y: 0 }, Point { x: 1, y: 0 }],
            Pipe::Vertical => &[Point { x: 0, y: -1 }, Point { x: 0, y: 1 }],
            Pipe::UpLeft => &[Point { x: 0, y: -1 }, Point { x: -1, y: 0 }],
            Pipe::UpRight => &[Point { x: 0, y: -1 }, Point { x: 1, y: 0 }],
            Pipe::DownLeft => &[Point { x: 0, y: 1 }, Point { x: -1, y: 0 }],
            Pipe::DownRight => &[Point { x: 0, y: 1 }, Point { x: 1, y: 0 }],
            Pipe::Snarbolax => panic!("Snarbolax is not connected!"),
        }
    }
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '.' => Some(Pipe::None),
            '-' => Some(Pipe::Horizontal),
            '|' => Some(Pipe::Vertical),
            'F' => Some(Pipe::DownRight),
            '7' => Some(Pipe::DownLeft),
            'L' => Some(Pipe::UpRight),
            'J' => Some(Pipe::UpLeft),
            'S' => Some(Pipe::Snarbolax),
            _ => None,
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Pipe::None => '·',
                Pipe::Horizontal => '─',
                Pipe::Vertical => '│',
                Pipe::UpLeft => '┘',
                Pipe::UpRight => '└',
                Pipe::DownLeft => '┐',
                Pipe::DownRight => '┌',
                Pipe::Snarbolax => '☺',
            } /*
              match self {
                  Pipe::None => '.',
                  Pipe::Horizontal => '-',
                  Pipe::Vertical => '|',
                  Pipe::UpLeft => 'J',
                  Pipe::UpRight => 'L',
                  Pipe::DownLeft => '7',
                  Pipe::DownRight => 'F',
                  Pipe::Snarbolax => 'S',
              }
              */
        )
    }
}

#[derive(Clone)]
struct Tilemap<T: Clone> {
    vec: Vec<T>,
    width: i32,
    height: i32,
}

impl<T: Clone> Tilemap<T> {
    pub fn new_empty() -> Tilemap<T> {
        Tilemap {
            vec: vec![],
            width: 0,
            height: 0,
        }
    }
    pub fn add_row(&mut self, row: &[T]) {
        if self.width == 0 {
            self.width = row.len() as i32;
        } else {
            assert_eq!(
                self.width,
                row.len() as i32,
                "Tried to add a row that was the wrong length!"
            );
        }
        self.vec.extend(row.iter().cloned());
        self.height += 1;
    }
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.vec.chunks_exact(self.width as usize)
    }
    pub fn find_tile(&self, predicate: impl Fn(&T) -> bool) -> Option<Point> {
        for (y, row) in self.rows().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if predicate(tile) {
                    return Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    }
    pub fn get_tile(&self, point: Point) -> Option<&T> {
        if point.x < 0
            || point.x >= self.width
            || point.y < 0
            || point.y >= self.height
        {
            None
        } else {
            Some(&self.vec[(point.x + point.y * self.width) as usize])
        }
    }
    pub fn set_tile(&mut self, point: Point, value: T) {
        if point.x < 0
            || point.x >= self.width
            || point.y < 0
            || point.y >= self.height
        {
            panic!("Can't set_tile outside the bounds of the map!")
        } else {
            self.vec[(point.x + point.y * self.width) as usize] = value;
        }
    }
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
}

impl<T: Clone + Display> Display for Tilemap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for row in self.rows() {
            for el in row.iter() {
                write!(f, "{el}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone + Default> Tilemap<T> {
    pub fn new(width: i32, height: i32) -> Tilemap<T> {
        Tilemap {
            vec: vec![
                Default::default();
                (width.checked_mul(height)).unwrap().try_into().unwrap()
            ],
            width,
            height,
        }
    }
}

/// Returns the location at which we found Snarbolax.
fn peer_through_snarbolax(tilemap: &mut Tilemap<Pipe>) -> Point {
    let snarbolax_pos = tilemap
        .find_tile(|x| *x == Pipe::Snarbolax)
        .expect("Where is my Snarbie!!??");
    let mut connectibles = Vec::with_capacity(4);
    for connection in ALL_CONNECTIONS {
        let neighbor = snarbolax_pos + *connection;
        if let Some(pipe) = tilemap.get_tile(neighbor) {
            if pipe.get_connections().iter().any(|x| *x == -*connection) {
                connectibles.push(*connection);
            }
        }
    }
    assert_eq!(
        connectibles.len(),
        2,
        "Snarbolax must be connected to exactly two pipes!"
    );
    let top = connectibles.iter().any(|x| *x == Point { x: 0, y: -1 });
    let bottom = connectibles.iter().any(|x| *x == Point { x: 0, y: 1 });
    let left = connectibles.iter().any(|x| *x == Point { x: -1, y: 0 });
    let right = connectibles.iter().any(|x| *x == Point { x: 1, y: 0 });
    let underpipe = match (top, bottom, left, right) {
        (true, true, false, false) => Pipe::Vertical,
        (false, false, true, true) => Pipe::Horizontal,
        (true, false, true, false) => Pipe::UpLeft,
        (false, true, false, true) => Pipe::DownRight,
        (true, false, false, true) => Pipe::UpRight,
        (false, true, true, false) => Pipe::DownLeft,
        _ => panic!("Snarbolax caused non-euclidean existence again: {top:?} {bottom:?} {left:?} {right:?}"),
    };
    tilemap.set_tile(snarbolax_pos, underpipe);
    // Make sure there was only one Snarbolax (dear lord)
    assert!(tilemap.find_tile(|x| *x == Pipe::Snarbolax).is_none());
    snarbolax_pos
}

fn follow_pipe(end: Point, prev: Point, tilemap: &Tilemap<Pipe>) -> Point {
    let pipe = tilemap.get_tile(end).unwrap();
    let connections = pipe.get_connections();
    assert_eq!(connections.len(), 2);
    if end + connections[0] == prev {
        end + connections[1]
    } else {
        assert_eq!(end + connections[1], prev);
        end + connections[0]
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut tilemap = Tilemap::new_empty();
    let mut buf: Vec<Pipe> = vec![];
    for line in lines {
        buf.clear();
        buf.extend(line.chars().map(Pipe::from_char).map(|x| x.unwrap()));
        tilemap.add_row(&buf);
    }
    let snarbolax_pos = peer_through_snarbolax(&mut tilemap);
    let snarbolax_pipe = *tilemap.get_tile(snarbolax_pos).unwrap();
    // Am I gonna regret that I mutated Snarbolax away...? NOPE!
    let mut loopmap = Tilemap::new(tilemap.get_width(), tilemap.get_height());
    loopmap.set_tile(snarbolax_pos, snarbolax_pipe);
    let ends = snarbolax_pipe.get_connections();
    assert_eq!(ends.len(), 2);
    let mut ends = [snarbolax_pos + ends[0], snarbolax_pos + ends[1]];
    let mut prevs = [snarbolax_pos, snarbolax_pos];
    let mut distance = 1;
    while ends[0] != ends[1] {
        loopmap.set_tile(ends[0], *tilemap.get_tile(ends[0]).unwrap());
        loopmap.set_tile(ends[1], *tilemap.get_tile(ends[1]).unwrap());
        (prevs[0], ends[0]) =
            (ends[0], follow_pipe(ends[0], prevs[0], &tilemap));
        (prevs[1], ends[1]) =
            (ends[1], follow_pipe(ends[1], prevs[1], &tilemap));
        distance += 1;
    }
    loopmap.set_tile(ends[0], *tilemap.get_tile(ends[0]).unwrap());
    println!("Puzzle 1 answer: {distance}");
    let mut total = 0;
    let mut insidemap = loopmap.clone();
    for (y, row) in loopmap.rows().enumerate() {
        let mut inside_up = false;
        let mut inside_down = false;
        for (x, pipe) in row.iter().enumerate() {
            match *pipe {
                Pipe::None => {
                    if inside_up && inside_down {
                        insidemap.set_tile(
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            Pipe::Snarbolax,
                        );
                        total += 1;
                    }
                }
                Pipe::Vertical => {
                    inside_up = !inside_up;
                    inside_down = !inside_down;
                }
                Pipe::UpLeft | Pipe::UpRight => inside_up = !inside_up,
                Pipe::DownLeft | Pipe::DownRight => inside_down = !inside_down,
                _ => (),
            }
        }
    }
    println!("{insidemap}");
    println!("Puzzle 2 answer: {total}");
}
