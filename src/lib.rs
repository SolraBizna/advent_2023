use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, Neg, Sub},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
