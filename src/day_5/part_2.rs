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
    /// Ideally these would always be non-overlapping, but we're currently not
    /// enforcing that.
    ranges: Vec<Interval>,
}

/// Left-inclusive; possibly empty.
#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Map {
    /// Invariant: these must *cover* the non-negative number line.
    range_maps: Vec<RangeMap>,
}

#[derive(Debug, Clone, Copy)]
struct RangeMap {
    input: Interval,
    output_start: i64,
}

impl Map {
    fn subset_image(&self, input: &Subset) -> Subset {
        let out = input
            .ranges
            .iter()
            .flat_map(|&r| self.range_image(r).ranges)
            .collect();
        Subset { ranges: out }
    }

    fn range_image(&self, input: Interval) -> Subset {
        let mut out = vec![];
        for rmap in &self.range_maps {
            let input_segment = Interval::intersection(rmap.input, input);
            if !input_segment.is_empty() {
                let segment_image = input_segment.translate(rmap.offset());
                out.push(segment_image);
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

    fn is_empty(self) -> bool {
        self.start >= self.end
    }

    fn translate(self, offset: i64) -> Self {
        Self {
            start: self.start + offset,
            end: self.end + offset,
        }
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
