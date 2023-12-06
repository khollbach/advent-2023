mod input;

use std::cmp::{max, min};

use anyhow::{Context, Result};

use super::input::Input as RawInput;

pub fn solve(input: &RawInput) -> Result<i64> {
    let input = Input::from_raw(input)?;

    let mut subset = input.initial_subset.clone();
    for map in &input.maps {
        subset = map.subset_image(&subset);
    }
    subset.min_value().context("empty subset")
}

#[derive(Debug)]
struct Input {
    initial_subset: Subset,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Subset {
    ranges: Vec<Interval>,
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Map {
    range_maps: Vec<RangeMap>,
}

#[derive(Debug, Clone, Copy)]
struct RangeMap {
    input: Interval,
    output_start: i64,
}

impl Map {
    fn subset_image(&self, subset: &Subset) -> Subset {
        let ranges = subset
            .ranges
            .iter()
            .flat_map(|&r| self.range_image(r).ranges)
            .collect();
        Subset { ranges }
    }

    fn range_image(&self, range: Interval) -> Subset {
        let mut out = vec![];
        for rmap in &self.range_maps {
            let mut segment = Interval::intersection(rmap.input, range);
            if !segment.is_empty() {
                segment.start += rmap.offset();
                out.push(segment);
            }
        }
        Subset { ranges: out }
    }
}

impl Interval {
    fn intersection(a: Self, b: Self) -> Self {
        let start = max(a.start, b.start);
        let end = min(a.end, b.end);
        Self { start, end }
    }

    fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

impl RangeMap {
    fn offset(self) -> i64 {
        self.output_start - self.input.start
    }
}

impl Subset {
    fn min_value(&self) -> Option<i64> {
        self.ranges.iter().map(|r| r.start).min()
    }
}
