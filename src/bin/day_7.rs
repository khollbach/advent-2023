use std::{cmp::Ordering, collections::HashMap, io};

use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let mut hands = read_input()?;
    // hands.sort_unstable();
    hands.sort_unstable_by_key(|&(h, _)| WildCardRules(h));

    let mut total_winnings = 0;
    for (i, (_, bid)) in (1..).zip(hands) {
        total_winnings += i * bid;
    }
    dbg!(total_winnings);

    Ok(())
}

type Line = (Hand, u32);
type Input = Vec<Line>;

fn read_input() -> Result<Input> {
    io::stdin().lines().map(parse_line).collect()
}

fn parse_line(line: io::Result<String>) -> Result<Line> {
    let line = line?;
    let (cards, bid) = line
        .split_once(char::is_whitespace)
        .context("split whitespace")?;
    let hand = parse_hand(cards)?;
    let line = (hand, bid.parse()?);
    Ok(line)
}

fn parse_hand(cards: &str) -> Result<Hand> {
    let cards: Vec<_> = cards.chars().map(parse_card).try_collect()?;
    let cards = cards
        .try_into()
        .map_err(|_| anyhow!("hand must contain 5 cards"))?;
    Ok(Hand { cards })
}

fn parse_card(c: char) -> Result<Card> {
    use Card::*;
    let card = match c {
        '2' => C2,
        '3' => C3,
        '4' => C4,
        '5' => C5,
        '6' => C6,
        '7' => C7,
        '8' => C8,
        '9' => C9,
        'T' => T,
        'J' => J,
        'Q' => Q,
        'K' => K,
        'A' => A,
        _ => bail!("not a card: {c:?}"),
    };
    Ok(card)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Break ties by lex ordering the card string.
        let key = |hand: &Self| (hand.category(), hand.cards);
        key(self).cmp(&key(other))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn category(self) -> Category {
        let mut freq_map = HashMap::new();
        for c in self.cards {
            *freq_map.entry(c).or_default() += 1;
        }
        let freqs: Vec<_> = freq_map.values().copied().collect();

        Category::from_freqs(&freqs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Category {
    fn from_freqs(freqs: &[u32]) -> Self {
        if freqs.contains(&5) {
            Category::FiveOfAKind
        } else if freqs.contains(&4) {
            Category::FourOfAKind
        } else if freqs.contains(&3) {
            if freqs.contains(&2) {
                Category::FullHouse
            } else {
                Category::ThreeOfAKind
            }
        } else if freqs.contains(&2) {
            let num_pairs = freqs.iter().filter(|&&f| f == 2).count();
            if num_pairs == 2 {
                Category::TwoPair
            } else {
                Category::OnePair
            }
        } else {
            Category::HighCard
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WildCardRules(Hand);

impl WildCardRules {
    fn category(self) -> Category {
        Category::from_freqs(&self.freqs())
    }

    fn freqs(self) -> Vec<u32> {
        let mut num_jokers = 0;
        let mut other_cards = HashMap::new();
        for c in self.0.cards {
            if c == Card::J {
                num_jokers += 1;
            } else {
                *other_cards.entry(c).or_default() += 1;
            }
        }

        let mut freqs: Vec<_> = other_cards.values().copied().sorted().collect();

        // All jokers.
        if freqs.is_empty() {
            return vec![5; 1];
        }

        // Jokers pretend to be the most frequent non-joker card.
        *freqs.last_mut().unwrap() += num_jokers;

        freqs
    }
}

impl Ord for WildCardRules {
    fn cmp(&self, other: &Self) -> Ordering {
        // Break ties by lex ordering the card string.
        let key = |wild: &Self| (wild.category(), wild.0.cards.map(WildCardOrdering));
        key(self).cmp(&key(other))
    }
}

impl PartialOrd for WildCardRules {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WildCardOrdering(Card);

impl Ord for WildCardOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        // Non-joker cards always rank higher.
        let key = |wild: &Self| (wild.0 != Card::J, wild.0);
        key(self).cmp(&key(other))
    }
}

impl PartialOrd for WildCardOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
