use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{cmp::{min, max}, io, result::Result as StdResult};

fn main() -> Result<()> {
    let input = read_input()?;

    let mut min_output = u64::MAX; // +infty
    for x in input.initial_seeds {
        let y = evaluate(&input.maps, x);
        min_output = min(min_output, y);
    }
    dbg!(min_output);

    Ok(())
}

fn evaluate(maps: &[Map], mut x: u64) -> u64 {
    for m in maps {
        x = m.eval(x);
    }
    x
}

/// We rely on the mappings appearing in-order in the input,
/// and don't validate this.
fn read_input() -> Result<Input> {
    let lines = io::read_to_string(io::stdin())?;
    let mut sections = lines.split("\n\n");

    let header = sections.next().context("header")?;
    let nums = header.strip_prefix("seeds: ").context("seeds")?;
    let initial_seeds = nums
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;

    let maps = sections.map(parse_map).collect::<Result<_>>()?;

    Ok(Input {
        initial_seeds,
        maps,
    })
}

// todo: validate no overlaps

fn parse_map(section: &str) -> Result<Map> {
    let mut lines = section.lines();

    let header = lines.next().context("map header")?;
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\w+-to-\w+ map:$").unwrap());
    ensure!(RE.is_match(header), "map header regex");

    let range_maps = lines.map(parse_range_map).collect::<Result<_>>()?;
    let map = Map { range_maps };
    ensure!(!has_overlap(&map), "overlap");
    Ok(map)
}

fn has_overlap(map: &Map) -> bool {
    let n = map.range_maps.len();
    for i in 0..n {
        for j in i + 1..n {
            if overlaps(&map.range_maps[i], &map.range_maps[j]) {
                return true;
            }
        }
    }
    false
}

fn overlaps(range1: &RangeMap, range2: &RangeMap) -> bool {
    let left1 = range1.src;
    let right1 = range1.src + range1.len;

    let left2 = range2.src;
    let right2 = range2.src + range2.len;

    max(left1, left2) < min(right1, right2)
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

struct Input {
    initial_seeds: Vec<u64>,
    maps: Vec<Map>,
}

struct Map {
    range_maps: Vec<RangeMap>,
}

#[derive(Debug, Clone, Copy)]
struct RangeMap {
    dest: u64,
    src: u64,
    len: u64,
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
