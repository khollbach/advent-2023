use std::{collections::BTreeSet, ops::Add};

use crate::day_18::{input::Dir, part_2::point::Point};

#[derive(Debug)]
pub struct Grid {
    /// Unique, sorted.
    x_values: Vec<i32>,
    /// Unique, sorted.
    y_values: Vec<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x_idx: isize,
    pub y_idx: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub bottom_left: GridCoord,
}

impl Grid {
    pub fn new(points: &[Point]) -> Self {
        // Unique, sorted.
        let x_values: BTreeSet<_> = points.iter().map(|p| p.x).collect();
        let y_values: BTreeSet<_> = points.iter().map(|p| p.y).collect();

        Self {
            x_values: x_values.into_iter().collect(),
            y_values: y_values.into_iter().collect(),
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.x_values.len(), self.y_values.len())
    }

    pub fn get(&self, gc: GridCoord) -> Point {
        assert!(gc.x_idx >= 0);
        assert!(gc.y_idx >= 0);
        let x = self.x_values[gc.x_idx as usize];
        let y = self.y_values[gc.y_idx as usize];
        Point { x, y }
    }

    pub fn find(&self, coord: Point) -> Option<GridCoord> {
        debug_assert!(self.x_values.windows(2).all(|pair| pair[0] < pair[1]));
        debug_assert!(self.y_values.windows(2).all(|pair| pair[0] < pair[1]));
        let x_idx = self.x_values.binary_search(&coord.x).ok()? as isize;
        let y_idx = self.y_values.binary_search(&coord.y).ok()? as isize;
        Some(GridCoord { x_idx, y_idx })
    }

    pub fn area(&self, tile: Tile) -> u64 {
        let top_right = tile.bottom_left + Dir::Up + Dir::Right;

        let b_l = self.get(tile.bottom_left);
        let t_r = self.get(top_right);

        let dx = t_r.x - b_l.x;
        let dy = t_r.y - b_l.y;
        debug_assert!(dx >= 0);
        debug_assert!(dy >= 0);

        dx as u64 * dy as u64
    }
}

impl Add<Dir> for GridCoord {
    type Output = Self;

    fn add(mut self, dir: Dir) -> Self {
        let Point { x, y } = dir.into();
        self.x_idx += x as isize;
        self.y_idx += y as isize;
        self
    }
}
