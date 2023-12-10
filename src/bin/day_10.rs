use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Add,
};

use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let (start, graph) = read_input()?;
    let n = graph.num_reachable_nodes(start);
    dbg!(n / 2);
    Ok(())
}

fn main() -> Result<()> {
    let (start, graph) = read_input()?;

    let ph1 = graph.phase_1(start);
    let ans = graph.phase_2(ph1);
    dbg!(ans);

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
            let p = (row, col).into();
            g.get_mut(p).tile = c;

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

            for d in dirs {
                let nbr = p + d;
                if g.in_bounds(nbr) {
                    g.get_mut(p).neighbors.push(nbr);
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
    tile: char,
    neighbors: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Left,
    Right,
}

#[derive(Debug, Default)]
struct Phase1 {
    /// Pipes in the main loop.
    seen: HashSet<Point>,
    /// Colors of perimeter tiles.
    colors: HashMap<Point, Color>,
}

// Part 2 code; phase 1.
impl Graph {
    /// Go once around the loop with two paint buckets.
    ///
    /// Paint everything on your left "blue" and everything on your right "red".
    /// Now the perimeter of every enclosed area has the correct color.
    fn phase_1(&self, start: Point) -> Phase1 {
        let mut out = Phase1::default();
        self.dfs(start, &mut out);
        for p in &out.seen {
            out.colors.remove(p);
        }
        out
    }

    fn dfs(&self, curr: Point, out: &mut Phase1) {
        if out.seen.contains(&curr) {
            return;
        }
        out.seen.insert(curr);

        for (color, offset) in self.paint_offsets(curr, &out.seen) {
            let p = curr + offset;
            if self.in_bounds(p) {
                out.colors.insert(p, color);
            }
        }

        for &nbr in &self.get(curr).neighbors {
            self.dfs(nbr, out);
        }
    }

    #[must_use]
    fn paint_offsets(&self, p: Point, seen: &HashSet<Point>) -> [(Color, Point); 2] {
        // Arbitrary choice of orientation, to be the "default" one.
        let arrow_head = match self.get(p).tile {
            '|' => UP,
            '-' => RIGHT,
            'L' => UP,
            'J' => LEFT,
            '7' => DOWN,
            'F' => RIGHT,
            _ => panic!("not a pipe"),
        };
        let inverted = seen.contains(&(p + arrow_head));

        // Assuming the default orientation, what would the output be?
        use Color::{Left as L, Right as R};
        let out = match self.get(p).tile {
            '|' => [(L, LEFT), (R, RIGHT)],
            '-' => [(L, UP), (R, DOWN)],
            'L' => [(L, DOWN), (L, LEFT)],
            'J' => [(L, RIGHT), (L, DOWN)],
            '7' => [(L, UP), (L, RIGHT)],
            'F' => [(L, LEFT), (L, UP)],
            _ => panic!("not a pipe"),
        };

        // Invert, if necessary.
        if inverted {
            out.map(|(color, dir)| (color.invert(), dir))
        } else {
            out
        }
    }
}

// Part 2 code; phase 2.
impl Graph {
    /// Perform an "MS Paint bucket-fill" on every blank region, based on the
    /// color of its perimeter.
    ///
    /// Return the total number of "blue" and "red" tiles.
    fn phase_2(&self, mut state: Phase1) -> HashMap<Color, usize> {
        for row in 0..self.dims().row {
            for col in 0..self.dims().col {
                let p = Point { row, col };

                if state.is_blank(p) {
                    let (points, color) = self.explore_region(p, &state.colors);
                    for p2 in points {
                        state.colors.insert(p2, color);
                    }
                }
            }
        }

        let mut color_freqs = HashMap::new();
        for &col in state.colors.values() {
            *color_freqs.entry(col).or_default() += 1;
        }
        color_freqs
    }

    fn explore_region(&self, p: Point, colors: &HashMap<Point, Color>) -> (HashSet<Point>, Color) {
        let mut seen = HashSet::new();
        let mut color = None;
        self.er_dfs(p, colors, &mut seen, &mut color);
        (seen, color.unwrap())
    }

    /// Helper for `explore_region`.
    fn er_dfs(
        &self,
        curr: Point,
        colors: &HashMap<Point, Color>,
        seen: &mut HashSet<Point>,
        perimeter_color: &mut Option<Color>,
    ) {
        if seen.contains(&curr) {
            return;
        }
        seen.insert(curr);

        for &nbr in &self.adjacent_tiles(curr) {
            if let Some(&color) = colors.get(&nbr) {
                // Consistency check: perimeter should be all the same color.
                if perimeter_color.is_some() {
                    assert!(*perimeter_color == Some(color));
                }

                *perimeter_color = Some(color);
            } else {
                self.er_dfs(nbr, colors, seen, perimeter_color);
            }
        }
    }
}

impl Phase1 {
    fn is_blank(&self, p: Point) -> bool {
        !self.seen.contains(&p) && !self.colors.contains_key(&p)
    }
}

impl Color {
    fn invert(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Graph {
    fn num_reachable_nodes(&self, start: Point) -> usize {
        let mut seen = HashSet::new();
        self.nrn_dfs(start, &mut seen);
        seen.len()
    }

    /// Helper for `num_reachable_nodes`.
    fn nrn_dfs(&self, curr: Point, seen: &mut HashSet<Point>) {
        if seen.contains(&curr) {
            return;
        }
        seen.insert(curr);

        for &nbr in &self.get(curr).neighbors {
            self.nrn_dfs(nbr, seen);
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

    fn adjacent_tiles(&self, p: Point) -> Vec<Point> {
        let mut out = vec![];
        for d in [UP, DOWN, LEFT, RIGHT] {
            let p2 = p + d;
            if self.in_bounds(p2) {
                out.push(p2);
            }
        }
        out
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
