use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    cmp::{max, min},
    io,
    result::Result as StdResult,
};

#[derive(Debug)]
pub struct RawInput {
    pub initial_seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Map {
    pub range_maps: Vec<RangeMap>,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeMap {
    pub dest: u64,
    pub src: u64,
    pub len: u64,
}

/// We rely on the mappings appearing in-order in the input,
/// and don't validate this.
pub fn read() -> Result<RawInput> {
    let lines = io::read_to_string(io::stdin())?;
    let mut sections = lines.split("\n\n");

    let header = sections.next().context("header")?;
    let nums = header.strip_prefix("seeds: ").context("seeds")?;
    let initial_seeds = nums
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;

    let maps = sections.map(parse_map).collect::<Result<_>>()?;

    Ok(RawInput {
        initial_seeds,
        maps,
    })
}

fn parse_map(section: &str) -> Result<Map> {
    let mut lines = section.lines();

    let header = lines.next().context("map header")?;
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\w+-to-\w+ map:$").unwrap());
    ensure!(RE.is_match(header), "map header regex");

    let range_maps = lines.map(parse_range_map).collect::<Result<_>>()?;
    let map = Map { range_maps };
    ensure!(!map.has_overlap(), "overlap");
    Ok(map)
}

fn parse_range_map(line: &str) -> Result<RangeMap> {
    let (dest, src, len) = line
        .split_whitespace()
        .map(str::parse)
        .collect_tuple()
        .context("dest src len")?;
    Ok(RangeMap {
        dest: dest?,
        src: src?,
        len: len?,
    })
}

impl Map {
    fn has_overlap(&self) -> bool {
        let n = self.range_maps.len();
        for i in 0..n {
            for j in i + 1..n {
                if self.range_maps[i].overlaps(&self.range_maps[j]) {
                    return true;
                }
            }
        }
        false
    }
}

impl RangeMap {
    fn overlaps(&self, other: &Self) -> bool {
        !is_empty(self.intersection(other))
    }

    fn intersection(&self, other: &Self) -> Interval {
        let start = max(self.src, other.src);
        let end = min(self.end(), other.end());
        (start, end)
    }

    fn end(&self) -> u64 {
        self.src + self.len
    }
}

type Interval = (u64, u64);

fn is_empty((start, end): Interval) -> bool {
    start >= end
}
