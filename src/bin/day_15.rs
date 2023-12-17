use std::io;

use anyhow::{ensure, Context, Result};

fn main() -> Result<()> {
    let mut sum = 0u32;
    for s in read_input()? {
        sum += HASH(&s) as u32;
    }
    dbg!(sum);
    Ok(())
}

fn read_input() -> Result<Vec<String>> {
    let mut lines = io::stdin().lines();
    let l = lines.next().context("empty")??;
    ensure!(lines.next().is_none(), "too many lines");
    Ok(l.split(',').map(String::from).collect())
}

#[allow(non_snake_case)]
fn HASH(s: &str) -> u8 {
    let mut curr_val = 0u32;
    for c in s.chars() {
        curr_val += c as u32;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val as u8
}
