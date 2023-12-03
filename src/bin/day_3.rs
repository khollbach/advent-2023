use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Add,
    result::Result as StdResult,
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn _part_1() -> Result<()> {
    let grid = read_grid()?;
    let numbers = grid.find_numbers()?;

    let mut sum = 0;
    for n in numbers {
        if grid.is_part_number(n) {
            sum += n.value;
        }
    }
    dbg!(sum);

    Ok(())
}

fn read_grid() -> Result<Grid> {
    let mut lines: Vec<_> = io::stdin().lines().collect::<StdResult<_, _>>()?;
    ensure!(!lines.is_empty(), "empty");
    ensure!(lines.iter().all(|l| l.len() == lines[0].len()), "jagged");

    // Pad with '.'s, so we can skip bounds checks.
    for l in &mut lines {
        l.insert(0, '.');
        l.push('.');
    }
    let blank_line = ".".repeat(lines[0].len());
    lines.insert(0, blank_line.clone());
    lines.push(blank_line);

    Ok(Grid { lines })
}

struct Grid {
    /// Padded with '.'s around the edges.
    lines: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    start: Point,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Grid {
    fn find_numbers(&self) -> Result<Vec<Number>> {
        let mut numbers = vec![];

        for (row, l) in self.lines.iter().enumerate() {
            static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

            for match_ in RE.find_iter(l) {
                let value = match_.as_str().parse().context("number value")?;
                let n = Number {
                    value,
                    start: Point {
                        row,
                        col: match_.start(),
                    },
                    len: match_.len(),
                };
                numbers.push(n);
            }
        }

        Ok(numbers)
    }

    fn is_part_number(&self, n: Number) -> bool {
        let left = Point {
            row: n.start.row,
            col: n.start.col - 1,
        };
        let right = Point {
            row: n.start.row,
            col: n.start.col + n.len,
        };

        let start = n.start.col - 1;
        let end = n.start.col + n.len + 1;
        let above = &self.lines[n.start.row - 1][start..end];
        let below = &self.lines[n.start.row + 1][start..end];

        is_symbol(self.get(left))
            || is_symbol(self.get(right))
            || above.chars().any(is_symbol)
            || below.chars().any(is_symbol)
    }

    fn get(&self, p: Point) -> char {
        self.lines[p.row][p.col..].chars().next().unwrap()
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

/// Part 2
fn main() -> Result<()> {
    let grid = read_grid()?;
    let numbers = grid.find_numbers()?;

    let map = number_map(&numbers);

    let mut sum = 0;
    for gear in grid.find_gears(&map) {
        let [i, j] = gear.number_ids;
        let gear_ratio = numbers[i].value * numbers[j].value;
        sum += gear_ratio;
    }
    dbg!(sum);

    Ok(())
}

/// If (p, i) is in the map, it means that numbers[i] covers point p.
type NumberMap = HashMap<Point, usize>;

fn number_map(numbers: &[Number]) -> NumberMap {
    let mut map = HashMap::new();
    for (id, n) in numbers.iter().enumerate() {
        for col in n.start.col..n.start.col + n.len {
            let p = Point {
                row: n.start.row,
                col,
            };
            map.insert(p, id);
        }
    }
    map
}

impl Grid {
    fn find_gears(&self, map: &NumberMap) -> Vec<Gear> {
        let mut gears = vec![];
        for (row, l) in self.lines.iter().enumerate() {
            for (col, c) in l.chars().enumerate() {
                if c == '*' {
                    let nbrs = self.neighboring_numbers(Point { row, col }, map);
                    if nbrs.len() == 2 {
                        let (i, j) = nbrs.into_iter().collect_tuple().unwrap();
                        gears.push(Gear { number_ids: [i, j] });
                    }
                }
            }
        }
        gears
    }

    /// Return a set of number ids.
    fn neighboring_numbers(&self, p: Point, map: &NumberMap) -> HashSet<usize> {
        let mut nbrs = HashSet::new();

        let top_left = Point {
            row: p.row - 1,
            col: p.col - 1,
        };
        for row in [0, 1, 2] {
            for col in [0, 1, 2] {
                let p2 = top_left + Point { row, col };
                if let Some(&id) = map.get(&p2) {
                    nbrs.insert(id);
                }
            }
        }

        nbrs
    }
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    number_ids: [usize; 2],
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
