use anyhow::{Context, Result};

use super::input::{Input, Map, RangeMap};

pub fn solve(input: &Input) -> Result<u64> {
    input
        .initial_seeds
        .iter()
        .map(|&x| eval_all(&input.maps, x))
        .min()
        .context("no initial seeds")
}

fn eval_all(maps: &[Map], mut x: u64) -> u64 {
    for m in maps {
        x = m.eval(x);
    }
    x
}

impl Map {
    fn eval(&self, x: u64) -> u64 {
        for r in &self.range_maps {
            if r.contains(x) {
                return r.map(x);
            }
        }
        x
    }
}

impl RangeMap {
    fn contains(self, x: u64) -> bool {
        self.src <= x && x < self.src + self.len
    }

    fn map(self, x: u64) -> u64 {
        assert!(self.contains(x));
        let offset = x - self.src;
        let y = self.dest + offset;
        y
    }
}
