use std::{
    collections::HashSet,
    ops::{Add, AddAssign}, io,
};

use anyhow::{Result, bail};
use itertools::Itertools;

fn main() -> Result<()> {
    let grid = read_input()?;
    let start = State {
        position: Point { row: 0, col: 0 },
        direction: Dir::Right,
    };
    let seen = grid.explore(start);

    let dims = grid.dims();
    for row in 0..dims.row {
        for col in 0..dims.col {
            if seen.contains(&Point { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    dbg!(seen.len());
    Ok(())
}

fn read_input() -> Result<Grid> {
    let grid = io::stdin().lines().map(parse_row).try_collect()?;
    Ok(Grid { grid })
}

fn parse_row(line: io::Result<String>) -> Result<Vec<Tile>> {
    line?.chars().map(Tile::new).collect()
}

impl Tile {
    fn new(c: char) -> Result<Self> {
        let t = match c {
            '.' => Self::Empty,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '-' => Self::Dash,
            '|' => Self::Bar,
            _ => bail!("invalid tile symbol: {c:?}"),
        };
        Ok(t)
    }
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Slash,
    Backslash,
    Dash,
    Bar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    direction: Dir,
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

impl Grid {
    fn explore(&self, start: State) -> HashSet<Point> {
        let mut seen = HashSet::new();
        self.dfs(start, &mut seen);
        seen.into_iter().map(|state| state.position).collect()
    }

    fn dfs(&self, curr: State, seen: &mut HashSet<State>) {
        if !self.in_bounds(curr.position) {
            return;
        }
        if seen.contains(&curr) {
            return;
        }
        seen.insert(curr);

        match (self.get(curr.position), curr.direction) {
            (Tile::Empty, _)
            | (Tile::Dash, Dir::Left | Dir::Right)
            | (Tile::Bar, Dir::Up | Dir::Down) => self.dfs(curr.continue_(), seen),

            (Tile::Dash, Dir::Up | Dir::Down) => {
                self.dfs(curr.left(), seen);
                self.dfs(curr.right(), seen);
            }
            (Tile::Bar, Dir::Left | Dir::Right) => {
                self.dfs(curr.up(), seen);
                self.dfs(curr.down(), seen);
            }

            (Tile::Slash, Dir::Up) => self.dfs(curr.right(), seen),
            (Tile::Slash, Dir::Down) => self.dfs(curr.left(), seen),
            (Tile::Slash, Dir::Left) => self.dfs(curr.down(), seen),
            (Tile::Slash, Dir::Right) => self.dfs(curr.up(), seen),

            (Tile::Backslash, Dir::Up) => self.dfs(curr.left(), seen),
            (Tile::Backslash, Dir::Down) => self.dfs(curr.right(), seen),
            (Tile::Backslash, Dir::Left) => self.dfs(curr.up(), seen),
            (Tile::Backslash, Dir::Right) => self.dfs(curr.down(), seen),
        }
    }

    fn get(&self, p: Point) -> Tile {
        assert!(self.in_bounds(p));
        self.grid[p.row as usize][p.col as usize]
    }

    fn in_bounds(&self, p: Point) -> bool {
        let dims = self.dims();
        let row = 0 <= p.row && p.row < dims.row;
        let col = 0 <= p.col && p.col < dims.col;
        row && col
    }

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }
}

impl State {
    fn up(mut self) -> Self {
        self.direction = Dir::Up;
        self.continue_()
    }

    fn down(mut self) -> Self {
        self.direction = Dir::Down;
        self.continue_()
    }

    fn left(mut self) -> Self {
        self.direction = Dir::Left;
        self.continue_()
    }

    fn right(mut self) -> Self {
        self.direction = Dir::Right;
        self.continue_()
    }

    fn continue_(mut self) -> Self {
        self.position += self.direction.into();
        self
    }
}

impl Point {
    const fn new(row: isize, col: isize) -> Self {
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
        match d {
            Dir::Up => Point::new(-1, 0),
            Dir::Down => Point::new(1, 0),
            Dir::Left => Point::new(0, -1),
            Dir::Right => Point::new(0, 1),
        }
    }
}
