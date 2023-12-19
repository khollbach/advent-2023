use std::collections::HashSet;

use crate::day_18::{input::Dir, part_2::grid::GridCoord};

use super::grid::{Grid, Tile};

/// The graph of walls.
///
/// The nodes are the coordinate-points in the grid.
///
/// There's an edge between two nodes if they're connected by a trench.
#[derive(Debug)]
pub struct Walls {
    /// Same dimensions as grid.
    ///
    /// Indexed as `nodes[y_idx][x_idx]`.
    nodes: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, Default)]
struct Node {
    edges: Vec<GridCoord>,
}

impl Walls {
    pub fn new(grid: &Grid, trench: &[GridCoord]) -> Self {
        let (dim_x, dim_y) = grid.dims();
        let mut this = Self {
            nodes: vec![vec![Node::default(); dim_x]; dim_y],
        };

        let n = trench.len();
        for i in 0..n {
            let (a, b) = if i == n - 1 {
                (trench[n - 1], trench[0])
            } else {
                (trench[i], trench[i + 1])
            };
            this.get_mut(a).edges.push(b);
            this.get_mut(b).edges.push(a);
        }

        this
    }

    pub fn enclosed_tiles(&self, start: Tile) -> HashSet<Tile> {
        let mut out = HashSet::new();
        self.dfs(start, &mut out);
        out
    }

    /// Helper for `enclosed_tiles`.
    fn dfs(&self, curr: Tile, out: &mut HashSet<Tile>) {
        if out.contains(&curr) {
            return;
        }
        out.insert(curr);

        // Perform bounds checks, and panic if out of bounds.
        // This avoids infinite loops when trying to guess-and-check a good
        // starting point.
        let _ = self.get(curr.bottom_left);

        for next in self.adjacent_tiles(curr) {
            self.dfs(next, out);
        }
    }

    fn adjacent_tiles(&self, tile: Tile) -> Vec<Tile> {
        let curr = tile.bottom_left;
        let up = curr + Dir::Up;
        let right = curr + Dir::Right;
        let up_right = curr + Dir::Up + Dir::Right;

        let mut dirs = vec![];
        if !self.has_edge(curr, up) {
            dirs.push(Dir::Left);
        }
        if !self.has_edge(curr, right) {
            dirs.push(Dir::Down);
        }
        if !self.has_edge(up, up_right) {
            dirs.push(Dir::Up);
        }
        if !self.has_edge(right, up_right) {
            dirs.push(Dir::Right);
        }

        dirs.into_iter()
            .map(|d| Tile {
                bottom_left: tile.bottom_left + d,
            })
            .collect()
    }

    fn has_edge(&self, a: GridCoord, b: GridCoord) -> bool {
        self.get(a).edges.contains(&b)
    }

    fn get(&self, coord: GridCoord) -> &Node {
        assert!(coord.x_idx >= 0);
        assert!(coord.y_idx >= 0);
        &self.nodes[coord.y_idx as usize][coord.x_idx as usize]
    }

    fn get_mut(&mut self, coord: GridCoord) -> &mut Node {
        assert!(coord.x_idx >= 0);
        assert!(coord.y_idx >= 0);
        &mut self.nodes[coord.y_idx as usize][coord.x_idx as usize]
    }
}
