use std::io;

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = read_input()?;
    let mut sum = 0;
    for mut l in lines {
        let orig_state = l.clone();
        sum += l.solve(0);
        debug_assert_eq!(l, orig_state);
    }
    dbg!(sum);
    Ok(())
}

fn read_input() -> Result<Vec<Line>> {
    io::stdin().lines().map(parse_line).collect()
}

fn parse_line(line: io::Result<String>) -> Result<Line> {
    let line = line?;
    let (conditions, group_lengths) = line.split_once(' ').context("space")?;
    let conditions = conditions.chars().map(Condition::new).collect();
    let group_lengths = group_lengths.split(',').map(str::parse).try_collect()?;
    Ok(Line {
        conditions,
        group_lengths,
    })
}

impl Condition {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("invalid condition symbol: {c:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    conditions: Vec<Condition>,
    group_lengths: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Line {
    fn solve(&mut self, i: usize) -> usize {
        // Base case: no unknowns.
        if i == self.conditions.len() {
            let gl = group_lengths(&self.conditions);
            return if gl == self.group_lengths { 1 } else { 0 };
        }

        // Brute force, try both possibilities.
        if self.conditions[i] == Condition::Unknown {
            let mut ans = 0;
            self.conditions[i] = Condition::Operational;
            ans += self.solve(i + 1);
            self.conditions[i] = Condition::Damaged;
            ans += self.solve(i + 1);
            self.conditions[i] = Condition::Unknown; // restore original state
            return ans;
        }

        // Happy path; keep scanning for unknowns.
        self.solve(i + 1)
    }
}

fn group_lengths(conditions: &[Condition]) -> Vec<usize> {
    conditions
        .iter()
        .group_by(|&&c| c)
        .into_iter()
        .filter_map(|(c, g)| match c {
            Condition::Operational => None,
            Condition::Damaged => Some(g.count()),
            Condition::Unknown => panic!("unknown condition"),
        })
        .collect()
}
