use anyhow::{ensure, Result};
use itertools::Itertools;

use crate::day_5::{
    input::{self as raw_input, Input as RawInput},
    part_2::{Input, Interval, Map, RangeMap, Subset},
};

impl Input {
    /// Massage types, to get the input in a pleasant shape for solving part 2.
    pub fn from_raw(input: &RawInput) -> Result<Input> {
        ensure!(input.initial_seeds.len() % 2 == 0, "odd number of seeds");

        let mut ranges = vec![];
        for pair in input.initial_seeds.chunks(2) {
            let &[start, len] = pair else { unreachable!() };
            ranges.push(Interval::from_start_len(start, len));
        }
        let initial_subset = Subset { ranges };

        let maps = input.maps.iter().map(Map::from_raw_input).collect();

        Ok(Input {
            initial_subset,
            maps,
        })
    }
}

impl Interval {
    fn from_start_len(start: u64, len: u64) -> Self {
        let end = start + len;
        Self {
            start: start as i64,
            end: end as i64,
        }
    }
}

impl Map {
    /// Fill out the map with identity RangeMaps, so that it covers the entire
    /// non-negative number line.
    fn from_raw_input(map: &raw_input::Map) -> Self {
        let range_maps = map
            .range_maps
            .iter()
            .map(RangeMap::from_raw_input)
            .collect();
        let mut this = Self { range_maps };
        this.fill_holes();
        this
    }

    fn fill_holes(&mut self) {
        let ranges = self.range_maps.iter().map(|r| r.input).collect();
        let covered = Subset { ranges };
        let exposed = covered.compliment();

        for r in exposed.ranges {
            self.range_maps.push(RangeMap::identity(r));
        }
    }
}

impl Subset {
    /// NOTE: this assumes the intervals of `self` are non-overlapping; but we
    /// don't actually check this.
    fn compliment(&self) -> Self {
        let points = self
            .ranges
            .iter()
            .flat_map(|r| [r.start, r.end])
            .chain([0, i64::MAX])
            .sorted();

        let mut out = vec![];
        for pair in &points.chunks(2) {
            let (start, end) = pair.collect_tuple().unwrap();
            let r = Interval { start, end };
            if !r.is_empty() {
                out.push(r);
            }
        }
        Self { ranges: out }
    }
}

impl RangeMap {
    fn from_raw_input(r: &raw_input::RangeMap) -> Self {
        Self {
            input: Interval::from_start_len(r.src, r.len),
            output_start: r.dest as i64,
        }
    }

    fn identity(range: Interval) -> Self {
        Self {
            input: range,
            output_start: range.start,
        }
    }
}
