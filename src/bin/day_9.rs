use std::io;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let lines = read_input()?;

    let mut sum = 0;
    for mut l in lines {
        // sum += extrapolate(&l);

        l.reverse();
        sum += extrapolate(&l);
    }
    dbg!(sum);

    Ok(())
}

fn read_input() -> Result<Vec<Vec<i32>>> {
    io::stdin().lines().map(parse_line).collect()
}

fn parse_line(line: io::Result<String>) -> Result<Vec<i32>> {
    Ok(line?.split_whitespace().map(str::parse).try_collect()?)
}

fn extrapolate(line: &[i32]) -> i32 {
    assert!(!line.is_empty());

    if line.iter().all(|&x| x == 0) {
        return 0;
    }

    let diffs: Vec<_> = line.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let new_diff = extrapolate(&diffs);
    line.last().unwrap() + new_diff
}
