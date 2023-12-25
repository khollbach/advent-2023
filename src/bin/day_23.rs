use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Add,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use rand::{distributions::WeightedIndex, prelude::*};

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let grid = read_grid()?;
    let (start, end) = grid.start_end();
    let ans = grid.longest_path(start, end).context("no path exists")?;
    dbg!(ans);
    Ok(())
}

impl Grid {
    fn start_end(&self) -> (Point, Point) {
        let dims = self.dims();
        let start = (0, 1);
        let end = (dims.row - 1, dims.col - 2);
        (start.into(), end.into())
    }

    fn dims(&self) -> Point {
        let row = self.grid.len() as isize;
        let col = self.grid[0].len() as isize;
        Point { row, col }
    }

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

impl Grid {
    fn in_bounds(&self, p: Point) -> bool {
        let row = 0 <= p.row && p.row < self.dims().row;
        let col = 0 <= p.col && p.col < self.dims().col;
        row && col
    }
}

const DIRS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

// ---

fn main() -> Result<()> {
    let grid = read_grid()?;
    let (start, end) = grid.start_end();
    let graph = Graph::from_grid(&grid);

    let mut best = 0;
    loop {
        if let Some(weight) = graph.walk(start, end) {
            if weight > best {
                println!("new best: {weight}");
                best = weight;
            }
        }
    }
}

impl Graph {
    fn walk(&self, start: Point, end: Point) -> Option<usize> {
        let mut path_weight = 0;
        let mut curr = start;
        let mut seen = HashSet::new();

        loop {
            assert!(!seen.contains(&curr));
            seen.insert(curr);

            if curr == end {
                return Some(path_weight);
            }

            let Some(e) = self.random_unseen_edge(curr, &seen) else {
                return None;
            };

            path_weight += e.weight;
            curr = e.dest;
        }
    }

    fn random_unseen_edge(&self, curr: Point, seen: &HashSet<Point>) -> Option<Edge> {
        let candidates: Vec<_> = self.nodes[&curr]
            .edges
            .iter()
            .copied()
            .filter(|e| !seen.contains(&e.dest))
            .collect();
        if candidates.is_empty() {
            return None;
        }

        let dist = WeightedIndex::new(candidates.iter().map(|e| e.weight)).unwrap();
        let idx = dist.sample(&mut rand::thread_rng());
        Some(candidates[idx])
    }
}

// ---

impl Graph {
    fn from_grid(grid: &Grid) -> Self {
        let mut nodes = HashMap::new();
        for row in 0..grid.dims().row {
            for col in 0..grid.dims().col {
                let p = Point { row, col };
                if grid.get(p) == '*' {
                    nodes.insert(p, Node::default());
                }
            }
        }

        let mut this = Self { nodes };

        let (start, _) = grid.start_end();
        let mut seen = HashSet::new();
        seen.insert(start);
        grid.dfs(start + DOWN, start, 0, &mut seen, &mut this);

        this
    }
}

impl Grid {
    /// Helper for `Graph::from_grid`.
    fn dfs(
        &self,
        curr: Point,
        mut prev_node: Point,
        mut curr_edge_weight: usize,
        seen: &mut HashSet<Point>,
        out: &mut Graph,
    ) {
        curr_edge_weight += 1;

        if self.get(curr) == '*' {
            // Add both edges.
            let (a, b) = (curr, prev_node);
            let weight = curr_edge_weight;
            out.nodes
                .get_mut(&a)
                .unwrap()
                .edges
                .push(Edge { weight, dest: b });
            out.nodes
                .get_mut(&b)
                .unwrap()
                .edges
                .push(Edge { weight, dest: a });

            prev_node = curr;
            curr_edge_weight = 0;
        }

        // Already explored my neighbors.
        if seen.contains(&curr) {
            return;
        }
        seen.insert(curr);

        for d in DIRS {
            let next = curr + d;
            if self.in_bounds(next) && self.get(next) != '#' && next != prev_node {
                self.dfs(next, prev_node, curr_edge_weight, seen, out);
            }
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<Point, Node>,
}

#[derive(Debug, Default)]
struct Node {
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    weight: usize,
    dest: Point,
}
