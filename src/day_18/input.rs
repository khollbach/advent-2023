use std::io;

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub distance: u32,
    pub direction: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn read() -> Result<Vec<Command>> {
    io::stdin().lines().map(parse_line).collect()
}

fn parse_line(line: io::Result<String>) -> Result<Command> {
    let line = line?;
    let (_dir, _len, color) = line
        .split_whitespace()
        .collect_tuple()
        .context("expected 3 words")?;

    let re = Lazy::new(|| {
        let dist = r"([[:xdigit:]]{5})";
        let dir_code = r"([[:xdigit:]])";
        let re = format!(r"^\(#{}{}\)$", dist, dir_code);
        Regex::new(&re).unwrap()
    });
    let caps = re.captures(&color).context("failed to match regex")?;

    let hex_string = format!("000{}", &caps[1]);
    let distance: [u8; 4] = hex::decode(hex_string).unwrap().try_into().unwrap();
    let distance = u32::from_be_bytes(distance);

    let direction = match &caps[2] {
        "0" => Dir::Right,
        "1" => Dir::Down,
        "2" => Dir::Left,
        "3" => Dir::Up,
        s => bail!("direction code: expected {{0, 1, 2, 3}}, got {s:?}"),
    };

    Ok(Command {
        distance,
        direction,
    })
}
