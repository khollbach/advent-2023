use std::{collections::HashMap, io};

use anyhow::{bail, ensure, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let (directions, graph) = read_input()?;
    let ans = traverse(&directions, &graph);
    dbg!(ans);
    Ok(())
}

fn main() -> Result<()> {
    let (directions, graph) = read_input()?;
    part_2(&directions, &graph)?;
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

fn traverse(directions: &[Direction], graph: &Graph) -> usize {
    let mut curr = "AAA";

    for (i, &d) in directions.iter().cycle().enumerate() {
        if curr == "ZZZ" {
            return i;
        }

        let edge = match d {
            Direction::Left => 0,
            Direction::Right => 1,
        };
        curr = &graph[curr][edge];
    }

    unreachable!();
}

/// Ok, just going based off intuition, I think the naive solution is not going
/// to work this time.
#[allow(dead_code)]
fn traverse_part_2(directions: &[Direction], graph: &Graph) -> usize {
    let mut curr: Vec<_> = graph.keys().filter(|s| s.ends_with('A')).collect();

    for (i, &d) in directions.iter().cycle().enumerate() {
        if curr.iter().all(|s| s.ends_with('Z')) {
            return i;
        }

        let edge = match d {
            Direction::Left => 0,
            Direction::Right => 1,
        };
        for node in &mut curr {
            *node = &graph[*node][edge];
        }
    }

    unreachable!();
}

/// Prints out some equations that can be solved by hand.
///
/// In particular, we want the LCM (least-common-multiple) of a few different
/// cycle lengths. You can plug this into wolfram alpha.
fn part_2(directions: &[Direction], graph: &Graph) -> Result<()> {
    for start in graph.keys().filter(|s| s.ends_with('A')) {
        let r = find_repetition(directions, graph, start);
        dbg!(r);
    }
    Ok(())
}

#[derive(Debug)]
// The compiler thinks these fields are never read, but it doesn't realize we're
// using the Debug impl to print them.
#[allow(dead_code)]
struct Repetition {
    stem_length: usize,
    cycle_length: usize,
    winning_steps: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State<'a> {
    node: &'a str,
    dir_idx: usize,
}

fn find_repetition(directions: &[Direction], graph: &Graph, start: &str) -> Repetition {
    let mut seen_states = HashMap::new();
    let mut winning_steps = vec![];

    let mut node = start;

    for (step, (dir_idx, &d)) in directions.iter().enumerate().cycle().enumerate() {
        let state = State { node, dir_idx };
        if let Some(&repeated_step) = seen_states.get(&state) {
            return Repetition {
                stem_length: repeated_step,
                cycle_length: step - repeated_step,
                winning_steps,
            };
        }
        seen_states.insert(state, step);

        if node.ends_with('Z') {
            winning_steps.push(step);
        }

        let edge = match d {
            Direction::Left => 0,
            Direction::Right => 1,
        };
        node = &graph[node][edge];
    }

    unreachable!();
}
