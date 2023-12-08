use std::{collections::HashMap, io};

use anyhow::{bail, ensure, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let (directions, graph) = read_input()?;
    let ans = traverse(&directions, &graph)?;
    dbg!(ans);
    Ok(())
}

type Input = (Vec<Direction>, Graph);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

type Graph = HashMap<String, [String; 2]>;

fn read_input() -> Result<Input> {
    let mut lines = io::stdin().lines();

    let directions = lines
        .next()
        .context("directions")??
        .chars()
        .map(Direction::from_char)
        .collect::<Result<_>>()?;

    let blank = lines.next().context("blank")??;
    ensure!(blank.is_empty(), "expected blank line");

    let mut graph = HashMap::new();
    for l in lines {
        static RE: Lazy<Regex> = Lazy::new(|| {
            let label = "([[:alpha:]]+)";
            let re = format!(r"^{label} = \({label}, {label}\)$");
            Regex::new(&re).unwrap()
        });

        let l = l?;
        let caps = RE.captures(&l).context("failed to match regex")?;
        let node = caps[1].to_owned();
        let left = caps[2].to_owned();
        let right = caps[3].to_owned();

        graph.insert(node, [left, right]);
    }

    Ok((directions, graph))
}

impl Direction {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("expected L or R, got {c:?}"),
        }
    }
}

fn traverse(directions: &[Direction], graph: &Graph) -> Result<usize> {
    let mut curr = "AAA";

    for (i, &d) in directions.iter().cycle().enumerate() {
        if curr == "ZZZ" {
            return Ok(i);
        }

        let edge = match d {
            Direction::Left => 0,
            Direction::Right => 1,
        };
        curr = &graph[curr][edge];
    }

    bail!("stuck in a cycle");
}
