use std::{collections::HashSet, io, ops::Add};

use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let (start, graph) = read_input()?;
    let n = graph.num_reachable_nodes(start);
    dbg!(n / 2);
    Ok(())
}

fn read_input() -> Result<(Point, Graph)> {
    let mut lines: Vec<_> = io::stdin().lines().try_collect()?;

    let start = find_start(&lines).context("no S")?;

    // In my specific input, the start location is on a "|".
    let col = start.col as usize;
    lines[start.row as usize].replace_range(col..col + 1, "|");

    let graph = parse_graph(&lines)?;

    Ok((start, graph))
}

fn find_start(lines: &[String]) -> Option<Point> {
    for (row, l) in lines.iter().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c == 'S' {
                return Some((row, col).into());
            }
        }
    }
    None
}

fn parse_graph(lines: &[String]) -> Result<Graph> {
    let nrows = lines.len();
    ensure!(nrows != 0, "no rows");
    let ncols = lines[0].len();
    ensure!(ncols != 0, "no cols");
    ensure!(lines.iter().all(|l| l.len() == ncols), "jagged");

    let nodes = vec![vec![Node::default(); ncols]; nrows];
    let mut g = Graph { nodes };

    for (row, l) in lines.iter().enumerate() {
        for (col, c) in l.chars().enumerate() {
            let dirs = match c {
                '|' => [UP, DOWN],
                '-' => [LEFT, RIGHT],
                'L' => [UP, RIGHT],
                'J' => [UP, LEFT],
                '7' => [LEFT, DOWN],
                'F' => [DOWN, RIGHT],
                '.' => continue,
                _ => bail!("unexpected tile: {c:?}"),
            };

            let p = (row, col).into();
            for d in dirs {
                let p2 = p + d;
                if g.in_bounds(p2) {
                    g.get_mut(p).neighbors.push(p2);
                }
            }
        }
    }

    Ok(g)
}

const UP: Point = Point { row: -1, col: 0 };

const DOWN: Point = Point { row: 1, col: 0 };

const LEFT: Point = Point { row: 0, col: -1 };

const RIGHT: Point = Point { row: 0, col: 1 };

struct Graph {
    /// Non-empty rectangle.
    nodes: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, Default)]
struct Node {
    neighbors: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Graph {
    fn num_reachable_nodes(&self, start: Point) -> usize {
        let mut seen = HashSet::new();
        self.dfs(start, &mut seen);
        seen.len()
    }

    fn dfs(&self, curr: Point, seen: &mut HashSet<Point>) {
        if seen.contains(&curr) {
            return;
        }
        seen.insert(curr);

        for &nbr in &self.get(curr).neighbors {
            self.dfs(nbr, seen);
        }
    }

    fn get(&self, p: Point) -> &Node {
        assert!(self.in_bounds(p));
        &self.nodes[p.row as usize][p.col as usize]
    }

    fn get_mut(&mut self, p: Point) -> &mut Node {
        assert!(self.in_bounds(p));
        &mut self.nodes[p.row as usize][p.col as usize]
    }

    fn in_bounds(&self, p: Point) -> bool {
        let dims = self.dims();
        let row = 0 <= p.row && p.row < dims.row;
        let col = 0 <= p.col && p.col < dims.col;
        row && col
    }

    fn dims(&self) -> Point {
        let row = self.nodes.len() as isize;
        let col = self.nodes[0].len() as isize;
        Point { row, col }
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

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }
}
