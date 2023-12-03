use std::{io, result::Result as StdResult};

use anyhow::{ensure, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
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

#[derive(Debug, Clone, Copy)]
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
