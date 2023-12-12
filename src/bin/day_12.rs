use std::io;

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = read_input()?;
    let mut sum = 0;
    for mut l in lines {
        l.conditions.push(Condition::Unknown);
        l.conditions = l.conditions.repeat(5);
        l.conditions.pop();

        l.group_lengths = l.group_lengths.repeat(5);

        sum += l.solve();
    }
    dbg!(sum);
    Ok(())
}

impl Line {
    fn solve(&mut self) -> usize {
        // Left-pad with '.' to make bounds-checking / edge-cases easier.
        self.conditions.insert(0, Condition::Operational);

        let num_islands = self.group_lengths.len();
        let num_conds = self.conditions.len();
        let mut ans = vec![vec![0; num_conds + 1]; num_islands + 1];

        for i in 0..=num_islands {
            for c in 0..=num_conds {
                // Base case: no islands and empty input string.
                if (i, c) == (0, 0) {
                    ans[0][0] = 1;
                    continue;
                }

                let mut ways = 0;

                // The two recursive cases are "use it, or don't".

                // Use it.
                if i != 0 {
                    // Make sure the pattern ".###" is compatible.
                    // (Using a number of #s equal to the current island length.)
                    let pat_len = 1 + self.group_lengths[i - 1];
                    if c >= pat_len
                        && self.conditions[c - pat_len] == '.'
                        && (c - pat_len + 1..=c - 1).all(|c2| self.conditions[c2] == '#')
                    {
                        ways += ans[i - 1][c - pat_len];
                    }
                }

                // Don't (at least not yet).
                if c != 0 && self.conditions[c - 1] == '.' {
                    ways += ans[i][c - 1];
                }

                ans[i][c] = ways;
            }
        }

        // Restore original state.
        self.conditions.remove(0);

        ans[num_islands][num_conds]
    }
}

impl Condition {
    fn to_char(self) -> char {
        match self {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        }
    }
}

impl PartialEq<char> for Condition {
    fn eq(&self, symbol: &char) -> bool {
        let this = self.to_char();

        // Treat ?s as wildcards.
        this == '?' || *symbol == '?' || this == *symbol
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

impl Line {
    #[allow(dead_code)]
    fn brute_force(&mut self, i: usize) -> usize {
        // Base case: no unknowns.
        if i == self.conditions.len() {
            let gl = group_lengths(&self.conditions);
            return if gl == self.group_lengths { 1 } else { 0 };
        }

        // Brute force, try both possibilities.
        if self.conditions[i] == Condition::Unknown {
            let mut ans = 0;
            self.conditions[i] = Condition::Operational;
            ans += self.brute_force(i + 1);
            self.conditions[i] = Condition::Damaged;
            ans += self.brute_force(i + 1);
            self.conditions[i] = Condition::Unknown; // restore original state
            return ans;
        }

        // Happy path; keep scanning for unknowns.
        self.brute_force(i + 1)
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
