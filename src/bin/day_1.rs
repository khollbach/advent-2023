use std::{cmp::Reverse, error::Error, io};

fn _part_1() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let a = line.chars().nth(first).unwrap();
        let b = line.chars().nth(last).unwrap();
        let ab: String = [a, b].into_iter().collect();
        let n: u32 = ab.parse().unwrap();
        sum += n;
    }
    dbg!(sum);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let a = first_digit(&line);
        let b = last_digit(&line);
        let ab: String = [a, b].into_iter().collect();
        let n: u32 = ab.parse().unwrap();
        sum += n;
    }
    dbg!(sum);
    Ok(())
}

const DIGITS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn first_digit(line: &str) -> char {
    let mut candidates = vec![];

    // Word digits.
    for (s, d) in DIGITS {
        if let Some(idx) = line.find(s) {
            candidates.push((idx, d));
        }
    }

    // Single-char digits.
    let idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let digit = line.chars().nth(idx).unwrap();
    candidates.push((idx, digit));

    // Take the smallest idx.
    candidates.sort();
    let (_, d) = candidates[0];
    d
}

fn last_digit(line: &str) -> char {
    let mut candidates = vec![];

    // Word digits.
    for (s, d) in DIGITS {
        if let Some(idx) = line.rfind(s) {
            candidates.push((idx, d));
        }
    }

    // Single-char digits.
    let idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let digit = line.chars().nth(idx).unwrap();
    candidates.push((idx, digit));

    // Take the largest idx.
    candidates.sort_by_key(|&pair| Reverse(pair));
    let (_, d) = candidates[0];
    d
}
