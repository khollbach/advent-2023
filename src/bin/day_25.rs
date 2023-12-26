use std::{
    collections::{HashMap, HashSet},
    io, iter,
};

use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, Default)]
struct Node {
    edges: HashSet<usize>,
}

struct MaxFlow {
    amount: usize,
    component_size: usize,
}

impl Graph {
    fn max_flow(mut self, source: usize, sink: usize) -> MaxFlow {
        assert_ne!(source, sink);

        for amount in 0.. {
            match self.find_path(source, sink) {
                Err(component_size) => {
                    return MaxFlow {
                        amount,
                        component_size,
                    }
                }
                Ok(path) => {
                    for pair in path.windows(2) {
                        let &[a, b] = pair else { unreachable!() };
                        assert!(self.nodes[a].edges.contains(&b));
                        if !self.nodes[b].edges.contains(&a) {
                            self.nodes[b].edges.insert(a);
                        } else {
                            self.nodes[a].edges.remove(&b);
                        }
                    }
                }
            }
        }

        unreachable!()
    }

    /// On failure, returns the number of reachable nodes.
    fn find_path(&self, start: usize, end: usize) -> Result<Vec<usize>, usize> {
        let mut seen = HashSet::new();
        match self.dfs(start, end, &mut seen) {
            Some(mut path) => {
                path.reverse();
                Ok(path)
            }
            None => Err(seen.len()),
        }
    }

    /// Helper for `find_path`.
    fn dfs(&self, curr: usize, target: usize, seen: &mut HashSet<usize>) -> Option<Vec<usize>> {
        if seen.contains(&curr) {
            return None;
        }
        seen.insert(curr);

        if curr == target {
            return Some(vec![target]);
        }

        for &next in &self.nodes[curr].edges {
            if let Some(mut path) = self.dfs(next, target, seen) {
                path.push(curr);
                return Some(path);
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let graph = read_input()?;
    let n = graph.nodes.len();

    let ans = loop {
        let source = rand_idx(n);
        let sink = loop {
            let idx = rand_idx(n);
            if idx != source {
                break idx;
            }
        };

        let flow = graph.clone().max_flow(source, sink);
        if dbg!(flow.amount) <= 3 {
            assert_eq!(flow.amount, 3);
            break flow.component_size * (n - flow.component_size);
        }
    };

    dbg!(ans);

    Ok(())
}

fn rand_idx(n: usize) -> usize {
    rand::random::<usize>() % n
}

fn read_input() -> Result<Graph> {
    let lines: Vec<_> = io::stdin().lines().map(parse_line).try_collect()?;
    let labels = labels(&lines);

    let mut nodes = vec![Node::default(); labels.len()];
    for l in lines {
        let a = l.first;
        for b in l.rest {
            nodes[labels[&a]].edges.insert(labels[&b]);
            nodes[labels[&b]].edges.insert(labels[&a]);
        }
    }

    Ok(Graph { nodes })
}

fn labels(lines: &[Line]) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    let mut n = 0;
    for line in lines {
        for l in iter::once(&line.first).chain(&line.rest) {
            if !out.contains_key(l) {
                out.insert(l.clone(), n);
                n += 1;
            }
        }
    }
    out
}

struct Line {
    first: String,
    rest: Vec<String>,
}

fn parse_line(line: io::Result<String>) -> Result<Line> {
    let line = line?;
    let (first, rest) = line.split_once(": ").context(":")?;
    let rest = rest.split_whitespace().map(str::to_owned).collect();
    Ok(Line {
        first: first.to_owned(),
        rest,
    })
}
