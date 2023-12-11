use std::{
    cmp::{max, min},
    io,
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let lines: Vec<_> = io::stdin().lines().try_collect()?;

    let mut stars = vec![];
    let mut empty_rows = vec![true; 140];
    let mut empty_cols = vec![true; 140];
    for (row, l) in lines.iter().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c == '#' {
                stars.push(Point::from((row, col)));
                empty_rows[row] = false;
                empty_cols[col] = false;
            }
        }
    }
    let n = stars.len();
    dbg!(n);
    dbg!(n_choose_2(n)); // ~100K = 10^5

    let mut total = 0;
    for i in 0..n {
        for j in i + 1..n {
            total += stars[i].manhattan_dist(stars[j]);

            // Account for expansion.
            let (min, max) = bounding_box(stars[i], stars[j]);
            for row in min.row..max.row {
                if empty_rows[row as usize] {
                    total += 1;
                }
            }
            for col in min.col..max.col {
                if empty_cols[col as usize] {
                    total += 1;
                }
            }
        }
    }
    dbg!(total);

    Ok(())
}

/// Returns (top_left, bot_right).
fn bounding_box(p1: Point, p2: Point) -> (Point, Point) {
    let top_left = Point {
        row: min(p1.row, p2.row),
        col: min(p1.col, p2.col),
    };
    let bot_right = Point {
        row: max(p1.row, p2.row),
        col: max(p1.col, p2.col),
    };
    (top_left, bot_right)
}

fn n_choose_2(n: usize) -> usize {
    n * (n - 1) / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }
}

impl Point {
    fn manhattan_dist(self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}
