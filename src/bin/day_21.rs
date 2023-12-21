use std::{collections::HashSet, io};

use anyhow::{bail, ensure, Context, Result};

fn main() -> Result<()> {
    let (map, start) = read_input()?;

    let mut reachable = HashSet::new();
    reachable.insert(start);
    for _ in 0..64 {
        reachable = map.step(reachable);
    }
    dbg!(reachable.len());

    Ok(())
}

impl Map {
    fn step(&self, curr: HashSet<Point>) -> HashSet<Point> {
        let mut out = HashSet::new();
        for mut p in curr {
            p.row -= 1;
            out.insert(p);
            p.row += 2;
            out.insert(p);
            p.row -= 1;

            p.col -= 1;
            out.insert(p);
            p.col += 2;
            out.insert(p);
        }
        out.retain(|&Point { row, col }| matches!(self.grid[row][col], Tile::Floor));
        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
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
                    start = Some(Point { row, col });
                    Tile::Floor
                }
                _ => bail!("invalid tile symbol: {c:?}"),
            };
        }
    }

    Ok((Map { grid }, start.context("no start")?))
}
