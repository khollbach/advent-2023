use std::io;

use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut ans = 0;
    for mut input in read_input()? {
        match input.find_reflection()? {
            Reflection::Row(r) => ans += r * 100,
            Reflection::Col(c) => ans += c,
        }
    }
    dbg!(ans);
    Ok(())
}

fn main() -> Result<()> {
    let mut ans = 0;
    for mut input in read_input()? {
        match input.find_new_reflection()? {
            Reflection::Row(r) => ans += r * 100,
            Reflection::Col(c) => ans += c,
        }
    }
    dbg!(ans);
    Ok(())
}

fn read_input() -> Result<Vec<Input>> {
    let stdin = io::read_to_string(io::stdin())?;
    stdin.split("\n\n").map(parse_grid).collect()
}

fn parse_grid(s: &str) -> Result<Input> {
    let grid = s.lines().map(parse_row).try_collect()?;
    Ok(Input { grid })
}

fn parse_row(line: &str) -> Result<Vec<bool>> {
    line.chars()
        .map(|c| match c {
            '.' => Ok(false),
            '#' => Ok(true),
            _ => bail!("not a . or a #: {c:?}"),
        })
        .collect()
}

struct Input {
    grid: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reflection {
    Row(usize),
    Col(usize),
}

impl Input {
    fn find_new_reflection(&mut self) -> Result<Reflection> {
        let existing = self.find_reflection().context("unmodified")?;

        let (nrows, ncols) = self.dims();
        for i in 0..nrows {
            for j in 0..ncols {
                self.grid[i][j] ^= true;
                for r in self.find_reflections() {
                    if r != existing {
                        return Ok(r);
                    }
                }
                self.grid[i][j] ^= true; // restore orig state
            }
        }

        bail!("failed to find smudge")
    }

    fn find_reflections(&mut self) -> Vec<Reflection> {
        let rows = self.find_row_reflections();
        let cols = self.find_col_reflections();

        let rows = rows.into_iter().map(Reflection::Row);
        let cols = cols.into_iter().map(Reflection::Col);
        rows.chain(cols).collect()
    }

    fn find_reflection(&mut self) -> Result<Reflection> {
        let out = self.find_reflections();
        ensure!(out.len() == 1, "expected exactly one line of reflection");
        Ok(out[0])
    }

    fn find_row_reflections(&mut self) -> Vec<usize> {
        let n = self.grid.len();
        let mut out = vec![];

        for second_half in [false, true] {
            for i in 1..=n / 2 {
                if self.is_mirrored_at(i) {
                    let row_idx = if second_half { n - i } else { i };
                    out.push(row_idx);
                }
            }
            self.grid.reverse(); // check the bottom half
        }

        out
    }

    /// Only works for row indices in the top half.
    fn is_mirrored_at(&mut self, row_idx: usize) -> bool {
        assert!(row_idx <= self.grid.len() / 2);
        let i = row_idx;
        self.grid[0..i].reverse();
        let out = self.grid[0..i] == self.grid[i..2 * i];
        self.grid[0..i].reverse(); // restore original state
        out
    }

    fn find_col_reflections(&mut self) -> Vec<usize> {
        self.transpose();
        let out = self.find_row_reflections();
        self.transpose();
        out
    }

    fn transpose(&mut self) {
        let (nrows, ncols) = self.dims();

        let mut out = vec![vec![false; nrows]; ncols]; // note the swap !
        for i in 0..nrows {
            for j in 0..ncols {
                out[j][i] = self.grid[i][j];
            }
        }
        self.grid = out;
    }

    fn dims(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        (nrows, ncols)
    }
}
