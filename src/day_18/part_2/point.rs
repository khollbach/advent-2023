use std::ops::{Add, AddAssign, Mul};

use crate::day_18::input::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<Dir> for Point {
    fn from(dir: Dir) -> Self {
        let (x, y) = match dir {
            Dir::Right => (1, 0),
            Dir::Left => (-1, 0),
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
        };
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(mut self, scalar: i32) -> Self {
        self.x *= scalar;
        self.y *= scalar;
        self
    }
}
