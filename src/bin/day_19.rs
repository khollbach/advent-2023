use std::{
    collections::HashMap, convert::Infallible, io, result::Result as StdResult, str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = read_input()?;

    let mut sum = 0;
    for i in input.accepted_items() {
        sum += i.x + i.m + i.a + i.s;
    }
    dbg!(sum);

    Ok(())
}

impl Input {
    fn accepted_items(&self) -> Vec<Item> {
        self.items
            .iter()
            .copied()
            .filter(|&i| self.should_accept(i))
            .collect()
    }

    fn should_accept(&self, item: Item) -> bool {
        self.process(item, &self.workflows["in"])
    }

    fn process(&self, item: Item, wf: &Workflow) -> bool {
        match wf.process(item) {
            Action::Reject => false,
            Action::Accept => true,
            Action::Send(label) => self.process(item, &self.workflows[&label]),
        }
    }
}

impl Workflow {
    fn process(&self, item: Item) -> Action {
        for r in &self.rules {
            if let Some(a) = r.process(item) {
                return a;
            }
        }
        self.default.clone()
    }
}

impl Rule {
    fn process(&self, item: Item) -> Option<Action> {
        if self.condition.apply(item) {
            Some(self.action.clone())
        } else {
            None
        }
    }
}

impl Condition {
    fn apply(&self, item: Item) -> bool {
        let val = match self.field {
            Field::X => item.x,
            Field::M => item.m,
            Field::A => item.a,
            Field::S => item.s,
        };
        match self.comparison {
            Comparison::Less => val < self.threshold,
            Comparison::Greater => val > self.threshold,
        }
    }
}

struct Input {
    workflows: HashMap<String, Workflow>,
    items: Vec<Item>,
}

#[derive(Clone, Copy)]
struct Item {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

struct Workflow {
    rules: Vec<Rule>,
    default: Action,
}

struct Rule {
    condition: Condition,
    action: Action,
}

struct Condition {
    field: Field,
    comparison: Comparison,
    threshold: u32,
}

enum Field {
    X,
    M,
    A,
    S,
}

enum Comparison {
    Less,
    Greater,
}

#[derive(Clone)]
enum Action {
    Reject,
    Accept,
    Send(String),
}

fn read_input() -> Result<Input> {
    io::read_to_string(io::stdin())?.parse()
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (workflows, items) = s.split_once("\n\n").context("blank line")?;
        Ok(Self {
            workflows: parse_workflows(workflows)?,
            items: parse_items(items)?,
        })
    }
}

fn parse_workflows(s: &str) -> Result<HashMap<String, Workflow>> {
    s.lines().map(parse_workflow).collect()
}

fn parse_items(s: &str) -> Result<Vec<Item>> {
    s.lines().map(str::parse).collect()
}

fn parse_workflow(line: &str) -> Result<(String, Workflow)> {
    let re = Lazy::new(|| Regex::new(r"^(\w+)\{(.*)\}$").unwrap());
    let caps = re.captures(line).context("wf regex")?;
    let label = caps[1].to_owned();
    let workflow = caps[2].parse()?;
    Ok((label, workflow))
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rules = s.split(',');
        let default = rules.next_back().context("default")?.parse()?;
        let rules = rules.map(str::parse).try_collect()?;
        Ok(Self { rules, default })
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (condition, action) = s.split_once(':').context("colon")?;
        Ok(Self {
            condition: condition.parse()?,
            action: action.parse()?,
        })
    }
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (field, comparison, threshold) = if s.contains('<') {
            let (a, b) = s.split_once('<').unwrap();
            (a, Comparison::Less, b)
        } else {
            let (a, b) = s.split_once('>').unwrap();
            (a, Comparison::Greater, b)
        };
        Ok(Self {
            field: field.parse()?,
            comparison,
            threshold: threshold.parse()?,
        })
    }
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let this = match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => bail!("invalid field: {s:?}"),
        };
        Ok(this)
    }
}

impl FromStr for Action {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let this = match s {
            "R" => Self::Reject,
            "A" => Self::Accept,
            _ => Self::Send(s.to_owned()),
        };
        Ok(this)
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Lazy::new(|| {
            let val = r"(\d+)";
            let re = format!(r"^\{{x={0},m={0},a={0},s={0}\}}$", val);
            Regex::new(&re).unwrap()
        });
        let caps = re.captures(s).context("item regex")?;
        Ok(Self {
            x: caps[1].parse()?,
            m: caps[2].parse()?,
            a: caps[3].parse()?,
            s: caps[4].parse()?,
        })
    }
}
