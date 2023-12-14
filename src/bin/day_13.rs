use std::io;

use anyhow::{bail, ensure, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let mut ans = 0;
    for mut input in read_input()? {
        let row = input.find_row_reflection();
        let col = input.find_col_reflection();
        ensure!(
            row.is_some() ^ col.is_some(),
            "expected exactly one line of reflection"
        );
        if let Some(row) = row {
            ans += 100 * row;
        }
        if let Some(col) = col {
            ans += col;
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

impl Input {
    fn find_row_reflection(&mut self) -> Option<usize> {
        let n = self.grid.len();
        let mut out = None;

        for second_half in [false, true] {
            for i in 1..=n / 2 {
                if self.is_mirrored_at(i) {
                    let row_idx = if second_half { n - i } else { i };
                    debug_assert!(out.is_none(), "expected at most one line of reflection");
                    out = Some(row_idx);
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

    fn find_col_reflection(&mut self) -> Option<usize> {
        self.transpose();
        let out = self.find_row_reflection();
        self.transpose();
        out
    }

    fn transpose(&mut self) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();

        let mut out = vec![vec![false; nrows]; ncols]; // note the swap !
        for i in 0..nrows {
            for j in 0..ncols {
                out[j][i] = self.grid[i][j];
            }
        }
        self.grid = out;
    }
}
