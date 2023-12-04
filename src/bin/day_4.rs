use std::{collections::HashSet, io, result::Result as StdResult};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cards = read_cards()?;

    let mut score = 0;
    for c in cards {
        score += c.score();
    }
    dbg!(score);

    Ok(())
}

fn read_cards() -> Result<Vec<Card>> {
    let mut cards = vec![];
    for l in io::stdin().lines() {
        cards.push(read_card(&l?)?);
    }
    Ok(cards)
}

fn read_card(l: &str) -> Result<Card> {
    let (_header, body) = l.split_once(": ").context("colon")?;
    let (left, right) = body.split_once(" | ").context("bar")?;
    let winning_numbers = left
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let numbers_you_have = right
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    Ok(Card {
        winning_numbers,
        numbers_you_have,
    })
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let mut count = 0;
        for n in &self.numbers_you_have {
            if self.winning_numbers.contains(n) {
                count += 1;
            }
        }
        if count != 0 {
            2u32.pow(count - 1)
        } else {
            0
        }
    }
}
