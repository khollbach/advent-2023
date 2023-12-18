use std::{
    collections::{BTreeSet, HashMap},
    io,
    ops::{Add, AddAssign, Sub},
};

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let graph = read_input()?;
    let start = (0, 0).into();
    let end = graph.dims() - (1, 1).into();
    let ans = graph.shortest_constrained_path(start, end);
    dbg!(ans);
    Ok(())
}

fn read_input() -> Result<Graph> {
    let grid = io::stdin().lines().map(parse_line).try_collect()?;
    Ok(Graph { grid })
}

fn parse_line(line: io::Result<String>) -> Result<Vec<u32>> {
    line?
        .chars()
        .map(|c| {
            c.to_digit(10)
                .with_context(|| format!("invalid digit: {c:?}"))
        })
        .collect()
}

struct Graph {
    grid: Vec<Vec<u32>>,
}

// todo: the priority queue impl really deserves to be hidden behind an API

impl Graph {
    /// "Cost-first search".
    fn shortest_constrained_path(&self, start: Point, target: Point) -> Option<u32> {
        let mut seen = HashMap::<State, Seen>::new();
        // Priority queue, ordered by distance estimate.
        let mut to_visit = BTreeSet::<PqElem>::new();

        // "See" the first node (from both possible initial directions).
        for direction in [Dir::Right, Dir::Down] {
            let start = PqElem {
                distance_estimate: 0,
                state: State {
                    position: start,
                    direction,
                    streak_length: 0,
                },
            };
            seen.insert(
                start.state,
                Seen::ToVisit {
                    distance_estimate: 0,
                },
            );
            to_visit.insert(start);
        }

        while let Some(curr) = to_visit.pop_first() {
            seen.insert(curr.state, Seen::Visited);

            if curr.state.position == target && curr.state.streak_length >= 4 {
                return Some(curr.distance_estimate);
            }

            for next in self.successors(curr.state) {
                let distance_estimate = curr.distance_estimate + self.get(next.position);

                match seen.get(&next) {
                    Some(Seen::Visited) => (),

                    None => {
                        seen.insert(next, Seen::ToVisit { distance_estimate });
                        to_visit.insert(PqElem {
                            distance_estimate,
                            state: next,
                        });
                    }

                    Some(&Seen::ToVisit {
                        distance_estimate: existing,
                    }) => {
                        if distance_estimate < existing {
                            // Remove the old entry.
                            to_visit.remove(&PqElem {
                                distance_estimate: existing,
                                state: next,
                            });

                            // Insert it again with updated priority.
                            seen.insert(next, Seen::ToVisit { distance_estimate });
                            to_visit.insert(PqElem {
                                distance_estimate,
                                state: next,
                            });
                        }
                    }
                }
            }
        }

        None
    }

    fn successors(&self, state: State) -> Vec<State> {
        let left = State {
            position: state.position,
            direction: state.direction.rotate_left(),
            streak_length: 0,
        }
        .forward();

        let right = State {
            position: state.position,
            direction: state.direction.rotate_right(),
            streak_length: 0,
        }
        .forward();

        let forward = state.forward();

        let mut out = vec![];
        if state.streak_length >= 4 {
            out.push(left);
            out.push(right);
        }
        if state.streak_length < 10 {
            out.push(forward);
        }

        out.retain(|next| self.in_bounds(next.position));
        out
    }

    #[allow(dead_code)]
    fn successors_part_1(&self, state: State) -> Vec<State> {
        let left = State {
            position: state.position,
            direction: state.direction.rotate_left(),
            streak_length: 0,
        }
        .forward();

        let right = State {
            position: state.position,
            direction: state.direction.rotate_right(),
            streak_length: 0,
        }
        .forward();

        let forward = state.forward();

        let mut out = vec![];
        for next in [left, right, forward] {
            if self.in_bounds(next.position) && next.streak_length <= 3 {
                out.push(next);
            }
        }
        out
    }

    fn get(&self, p: Point) -> u32 {
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

/// The status of a node that has been seen during a dfs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seen {
    Visited,
    ToVisit { distance_estimate: u32 },
}

/// Element in a priority queue.
//
// We're being sloppy and deriving Ord for everything, even though it doesn't
// make sense for State, Point, or Dir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PqElem {
    distance_estimate: u32,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    position: Point,
    direction: Dir,
    streak_length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl State {
    fn forward(mut self) -> Self {
        self.position += self.direction.into();
        self.streak_length += 1;
        self
    }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
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

impl Dir {
    fn rotate_left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}
