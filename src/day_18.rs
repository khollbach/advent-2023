mod input;
mod part_2;

use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = input::read()?;
    let ans = part_2::solve(&input);
    dbg!(ans);
    Ok(())
}
