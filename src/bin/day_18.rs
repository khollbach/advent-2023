use std::{
    collections::HashSet,
    io,
    ops::{Add, AddAssign},
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input()?;
    let ans = enclosed_area(&input);
    dbg!(ans);
    Ok(())
}

type Input = Vec<Command>;
type Command = (Dir, usize);

fn read_input() -> Result<Input> {
    io::stdin().lines().map(parse_line).collect()
}

fn parse_line(line: io::Result<String>) -> Result<Command> {
    let line = line?;
    let (dir, len, _color) = line
        .split_whitespace()
        .collect_tuple()
        .context("expected 3 words")?;

    let dir = match dir {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => bail!("expected UDLR, got {dir:?}"),
    };

    let len = len.parse()?;

    Ok((dir, len))
}

fn enclosed_area(input: &[Command]) -> usize {
    let mut walls = HashSet::new();

    let mut curr = Point::from((0, 0));
    for &(dir, len) in input {
        for _ in 0..len {
            curr += dir.into();
            walls.insert(curr);
        }
    }

    // Paint-bucket-fill the middle. We guess-and-checked a good starting point.
    let start = (1, 1).into();
    let mut inner = HashSet::new();
    dfs(start, &walls, &mut inner);
    walls.len() + inner.len()
}

fn dfs(curr: Point, walls: &HashSet<Point>, visited: &mut HashSet<Point>) {
    if walls.contains(&curr) || visited.contains(&curr) {
        return;
    }
    visited.insert(curr);

    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let next = curr + dir.into();
        dfs(next, walls, visited);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<(isize, isize)> for Point {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl From<Dir> for Point {
    fn from(d: Dir) -> Self {
        let p = match d {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        p.into()
    }
}
