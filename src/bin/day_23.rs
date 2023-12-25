use std::{io, ops::Add};

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let grid = read_grid()?;

    let start = (0, 1).into();
    let end = (140, 139).into();

    let ans = grid.longest_path(start, end).context("no path exists")?;
    dbg!(ans);

    Ok(())
}

impl Grid {
    fn longest_path(&self, source: Point, dest: Point) -> Option<usize> {
        let mut paths = vec![];
        self.all_paths(&mut vec![source], dest, &mut paths);
        paths.into_iter().map(|p| p.len() - 1).max() // don't count the start node
    }

    fn all_paths(
        &self,
        curr_path: &mut Vec<Point>,
        target: Point,
        out_paths: &mut Vec<Vec<Point>>,
    ) {
        let curr = *curr_path.last().unwrap();

        if curr == target {
            out_paths.push(curr_path.clone());
        }

        for nbr in self.neighbors(curr) {
            if !curr_path.contains(&nbr) {
                curr_path.push(nbr);
                self.all_paths(curr_path, target, out_paths);
                assert_eq!(curr_path.pop(), Some(nbr));
            }
        }
    }

    fn neighbors(&self, p: Point) -> Vec<Point> {
        if p.row == 0 {
            return vec![p + DOWN];
        }
        if p.row == 140 {
            return vec![p + UP];
        }

        let dirs = match self.get(p) {
            '^' => vec![UP],
            'v' => vec![DOWN],
            '<' => vec![LEFT],
            '>' => vec![RIGHT],
            '.' => vec![UP, DOWN, LEFT, RIGHT],
            c => panic!("how am I on grid tile: {c:?} ??"),
        };

        let mut out = vec![];
        for d in dirs {
            if self.get(p + d) != '#' {
                out.push(p + d);
            }
        }
        out
    }

    fn get(&self, p: Point) -> char {
        assert!(p.row >= 0);
        assert!(p.col >= 0);
        self.grid[p.row as usize][p.col as usize] as char
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

const UP: Point = Point { row: -1, col: 0 };
const DOWN: Point = Point { row: 1, col: 0 };
const LEFT: Point = Point { row: 0, col: -1 };
const RIGHT: Point = Point { row: 0, col: 1 };

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}

struct Grid {
    grid: Vec<Vec<u8>>,
}

fn read_grid() -> Result<Grid> {
    let grid = io::stdin()
        .lines()
        .map_ok(|l| l.into_bytes())
        .try_collect()?;
    Ok(Grid { grid })
}
