use std::{collections::HashSet, io, ops::Add};

use anyhow::{bail, ensure, Context, Result};

fn main() -> Result<()> {
    let (map, start) = read_input()?;
    ensure!(start.row == 65);
    ensure!(start.col == 65);

    let mut total = 0u64;

    // Main checker-board area.
    total += 202_299_u64.pow(2) * map.reachable((65, 65), 1_001); // starting tile, e.g.
    total += 202_300_u64.pow(2) * map.reachable((65, 65), 1_000);

    // Four points: NESW.
    for p in [(130, 65), (65, 0), (0, 65), (65, 130)] {
        total += map.reachable(p, 130);
    }

    for p in [(0, 0), (0, 130), (130, 130), (130, 0)] {
        // Farther edge-pieces.
        total += 202_300 * map.reachable(p, 64);

        // Closer edge-pieces.
        total += 202_299 * map.reachable(p, 130 + 65);
    }

    dbg!(total);

    Ok(())
}

impl Map {
    fn reachable(&self, p: impl Into<Point>, num_steps: usize) -> u64 {
        let mut reachable = HashSet::new();
        reachable.insert(p.into());
        for _ in 0..num_steps {
            reachable = self.step(reachable);
        }
        reachable.len() as u64
    }
}

impl From<(isize, isize)> for Point {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let (map, start) = read_input()?;
    dbg!(map.reachable(start, 64));
    Ok(())
}

impl Map {
    fn step(&self, curr: HashSet<Point>) -> HashSet<Point> {
        let mut out = HashSet::new();
        for p in curr {
            for d in DIRS {
                let nbr = p + d;
                if matches!(self.get(nbr), Some(Tile::Floor)) {
                    out.insert(nbr);
                }
            }
        }
        out
    }

    fn get(&self, p: Point) -> Option<Tile> {
        if self.in_bounds(p) {
            Some(self.grid[p.row as usize][p.col as usize])
        } else {
            None
        }
    }

    fn in_bounds(&self, p: Point) -> bool {
        let row = 0 <= p.row && p.row <= 130;
        let col = 0 <= p.col && p.col <= 130;
        row && col
    }
}

const DIRS: [Point; 4] = [
    Point { row: -1, col: 0 },
    Point { row: 1, col: 0 },
    Point { row: 0, col: -1 },
    Point { row: 0, col: 1 },
];

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

struct Map {
    grid: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy)]
enum Tile {
    Floor,
    Wall,
}

fn read_input() -> Result<(Map, Point)> {
    let mut grid = vec![vec![Tile::Floor; 131]; 131];
    let mut start = None;

    for (row, l) in io::stdin().lines().enumerate() {
        for (col, c) in l?.chars().enumerate() {
            grid[row][col] = match c {
                '.' => Tile::Floor,
                '#' => Tile::Wall,
                'S' => {
                    ensure!(start.is_none(), "two starts");
                    start = Some((row as isize, col as isize).into());
                    Tile::Floor
                }
                _ => bail!("invalid tile symbol: {c:?}"),
            };
        }
    }

    Ok((Map { grid }, start.context("no start")?))
}
