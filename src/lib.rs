use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Self::Output {
        let rhs: Point = rhs.into();
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Point;
    fn sub(self, rhs: Direction) -> Self::Output {
        let rhs: Point = rhs.into();
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    pub const NORTH: Point = Point { x: 0, y: -1 };
    pub const SOUTH: Point = Point { x: 0, y: 1 };
    pub const WEST: Point = Point { x: -1, y: 0 };
    pub const EAST: Point = Point { x: 1, y: 0 };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const ALL: &[Direction] = &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    pub fn to_ordinal(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }
    pub fn to_arrow(&self) -> char {
        match self {
            Direction::North => '↑',
            Direction::South => '↓',
            Direction::East => '→',
            Direction::West => '←',
        }
    }
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl From<Point> for Direction {
    fn from(value: Point) -> Self {
        match value {
            Point::NORTH => Direction::North,
            Point::SOUTH => Direction::South,
            Point::EAST => Direction::East,
            Point::WEST => Direction::West,
            _ => panic!("cannot make a Direction from a non-orthogonal Point"),
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Point::NORTH,
            Direction::South => Point::SOUTH,
            Direction::East => Point::EAST,
            Direction::West => Point::WEST,
        }
    }
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Mul<i32> for Direction {
    type Output = Point;
    fn mul(self, rhs: i32) -> Point {
        Point::from(self) * rhs
    }
}

#[derive(Clone)]
pub struct Tilemap<T: Clone> {
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
    pub fn new_with(wat: T, width: i32, height: i32) -> Tilemap<T> {
        Tilemap {
            vec: vec![
                wat;
                (width.checked_mul(height)).unwrap().try_into().unwrap()
            ],
            width,
            height,
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
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.vec.chunks_exact_mut(self.width as usize)
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.vec.iter_mut()
    }
    pub fn get_row(&self, y: i32) -> Option<&[T]> {
        if y < 0 || y >= self.height {
            None
        } else {
            Some(
                &self.vec[(y * self.width) as usize
                    ..((y + 1) * self.width) as usize],
            )
        }
    }
    pub fn get_row_mut(&mut self, y: i32) -> Option<&mut [T]> {
        if y < 0 || y >= self.height {
            None
        } else {
            Some(
                &mut self.vec[(y * self.width) as usize
                    ..((y + 1) * self.width) as usize],
            )
        }
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
    pub fn find_tiles<'a>(
        &'a self,
        predicate: impl 'a + Clone + Copy + Fn(&T) -> bool,
    ) -> impl 'a + Iterator<Item = Point> {
        self.rows().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if predicate(tile) {
                    Some(Point {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
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
    pub fn get_tile_mut(&mut self, point: Point) -> Option<&mut T> {
        if point.x < 0
            || point.x >= self.width
            || point.y < 0
            || point.y >= self.height
        {
            None
        } else {
            Some(&mut self.vec[(point.x + point.y * self.width) as usize])
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
    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn transpose(&self) -> Tilemap<T> {
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for x in 0..self.width {
            for y in 0..self.height {
                new_vec.push(self.get_tile(Point { x, y }).unwrap().clone())
            }
        }
        Tilemap {
            vec: new_vec,
            width: self.height,
            height: self.width,
        }
    }
    pub fn flip_h(&self) -> Tilemap<T> {
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                new_vec.push(self.get_tile(Point { x, y }).unwrap().clone())
            }
        }
        Tilemap {
            vec: new_vec,
            ..*self
        }
    }
    pub fn flip_v(&self) -> Tilemap<T> {
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                new_vec.push(self.get_tile(Point { x, y }).unwrap().clone())
            }
        }
        Tilemap {
            vec: new_vec,
            ..*self
        }
    }
    pub fn rotate_cw(&self) -> Tilemap<T> {
        self.transpose().flip_h()
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
    pub fn insert_blank_row(&mut self, new_y: i32) {
        assert!(new_y >= 0 && new_y <= self.height);
        let first_index = (new_y * self.width) as usize;
        self.vec.splice(
            first_index..first_index,
            std::iter::repeat_with(Default::default).take(self.width as usize),
        );
        self.height += 1;
    }
    pub fn insert_blank_column(&mut self, new_x: i32) {
        assert!(new_x >= 0 && new_x <= self.width);
        self.width += 1;
        for y in 0..self.height {
            self.vec
                .insert((new_x + y * self.width) as usize, Default::default());
        }
    }
}

impl<T: Clone + Hash> Hash for Tilemap<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vec.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<T: Clone + PartialEq> PartialEq for Tilemap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
            && self.width == other.width
            && self.height == other.height
    }
}

impl<T: Clone + Eq> Eq for Tilemap<T> {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Self::Output {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Point3 {
    type Output = Point3;
    fn add(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: i32) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Point3 {
    /// Returns this point, but with each coordinate replaced with -1, 0, or 1
    /// according to its previous sign.
    pub fn unit(&self) -> Point3 {
        Point3 {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
    pub fn from_str_or_panic(s: &str) -> Point3 {
        let mut split = s.split(',');
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let z = split.next().unwrap();
        assert!(split.next().is_none());
        Point3 {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Neg for Point3f {
    type Output = Point3f;
    fn neg(self) -> Self::Output {
        Point3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Point3f {
    type Output = Point3f;
    fn add(self, rhs: Self) -> Self::Output {
        Point3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3f {
    type Output = Point3f;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Point3f {
    type Output = Point3f;
    fn mul(self, rhs: f64) -> Self::Output {
        Point3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Point3f {
    pub fn from_str_or_panic(s: &str) -> Point3f {
        let mut split = s.split(',');
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let z = split.next().unwrap();
        assert!(split.next().is_none());
        Point3f {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
