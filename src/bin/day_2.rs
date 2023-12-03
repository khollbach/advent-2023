use std::io;

use anyhow::{bail, ensure, Context, Result};

fn _part_1() -> Result<()> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let game = parse_line(&line?)?;
        if game.is_possible() {
            sum += game.id;
        }
    }
    dbg!(sum);
    Ok(())
}

fn main() -> Result<()> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let game = parse_line(&line?)?;
        let subset = game.required_supply();
        sum += power(subset);
    }
    dbg!(sum);
    Ok(())
}

type Subset = (u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn parse_line(line: &str) -> Result<Game> {
    let (left, right) = line.split_once(": ").context("colon")?;
    let id = left.strip_prefix("Game ").context("Game")?;
    let id = id.parse().context("id")?;

    let mut subsets = vec![];
    for s in right.split("; ") {
        subsets.push(parse_subset(s)?);
    }

    Ok(Game { id, subsets })
}

fn parse_subset(s: &str) -> Result<Subset> {
    let mut r = None;
    let mut g = None;
    let mut b = None;

    for phrase in s.split(", ") {
        let (left, right) = phrase.split_once(' ').context("space")?;

        let amount = left.parse().context("amount")?;
        let color = match right {
            "red" => &mut r,
            "green" => &mut g,
            "blue" => &mut b,
            _ => bail!("unknown color word: {right:?}"),
        };

        ensure!(color.is_none(), "re-defined color: {right:?}");
        *color = Some(amount);
    }

    Ok((r.unwrap_or(0), g.unwrap_or(0), b.unwrap_or(0)))
}

impl Game {
    fn is_possible(&self) -> bool {
        self.subsets
            .iter()
            .all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14)
    }

    fn required_supply(&self) -> Subset {
        let r = self.subsets.iter().map(|s| s.0).max().unwrap_or(0);
        let g = self.subsets.iter().map(|s| s.1).max().unwrap_or(0);
        let b = self.subsets.iter().map(|s| s.2).max().unwrap_or(0);
        (r, g, b)
    }
}

fn power((r, g, b): Subset) -> u32 {
    r * g * b
}
