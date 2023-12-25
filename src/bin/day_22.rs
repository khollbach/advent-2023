use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let world = read_input()?;
    dbg!(world.settle().num_safe());
    Ok(())
}

fn read_input() -> Result<World> {
    let mut out = World::default();

    for (i, l) in io::stdin().lines().enumerate() {
        let l = l?;
        let (p1, p2) = l.split_once('~').context("~")?;

        let p1: Point = p1.parse()?;
        let p2: Point = p2.parse()?;
        let mut points = vec![];

        let delta = (p2 - p1).sgn();
        let mut p = p1;
        loop {
            points.push(p);
            if p == p2 {
                break;
            }
            p += delta;
        }

        out.insert(Brick {
            id: BrickId(i),
            points,
        });
    }

    Ok(out)
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let coords: Vec<_> = s.split(',').map(str::parse).try_collect()?;
        let &[x, y, z] = coords.as_slice() else {
            bail!("triple");
        };
        Ok(Self { x, y, z })
    }
}

impl Point {
    fn sgn(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

// ---

#[derive(Default)]
struct World {
    bricks: HashMap<BrickId, Brick>,
    space: HashMap<Point, BrickId>,
}

struct Brick {
    id: BrickId,
    points: Vec<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BrickId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl World {
    fn settle(mut self) -> Self {
        let mut out = Self::default();

        // Drop the lowest bricks first.
        for z in 1..=400 {
            for x in 0..10 {
                for y in 0..10 {
                    let p = Point { x, y, z };
                    if let Some(&b) = self.space.get(&p) {
                        let brick = self.remove(b);
                        out.drop(brick);
                    }
                }
            }
        }

        out
    }

    fn insert(&mut self, b: Brick) {
        for &p in &b.points {
            let old = self.space.insert(p, b.id);
            assert!(old.is_none());
        }
        let old = self.bricks.insert(b.id, b);
        assert!(old.is_none());
    }

    fn remove(&mut self, b: BrickId) -> Brick {
        for &p in &self.bricks[&b].points {
            let ret = self.space.remove(&p);
            assert_eq!(ret, Some(b));
        }
        self.bricks.remove(&b).unwrap()
    }

    fn drop(&mut self, mut b: Brick) {
        assert!(!self.bricks.contains_key(&b.id));

        let mut highest = i32::MIN;
        let x: HashSet<_> = b.points.iter().map(|p| p.x).collect();
        let y: HashSet<_> = b.points.iter().map(|p| p.y).collect();
        for &x in &x {
            for &y in &y {
                highest = max(highest, self.highest_xy(x, y));
            }
        }

        let z = highest + 1;
        b.set_height(z);

        self.insert(b);
    }

    fn highest_xy(&self, x: i32, y: i32) -> i32 {
        for z in (1..=300).rev() {
            let p = Point { x, y, z };
            if self.space.contains_key(&p) {
                return z;
            }
        }
        0
    }
}

impl Brick {
    /// Set the height of the *lowest* block in this brick.
    fn set_height(&mut self, mut z: i32) {
        let vert = self.is_vertical();

        self.points.sort_by_key(|p| p.z);
        for p in &mut self.points {
            p.z = z;
            if vert {
                z += 1;
            }
        }
    }

    /// Is this brick tall and skinny?
    fn is_vertical(&self) -> bool {
        let z: HashSet<_> = self.points.iter().map(|p| p.z).collect();
        z.len() >= 2
    }
}

// ---

const DOWN: Point = Point { x: 0, y: 0, z: -1 };

impl World {
    fn num_safe(&self) -> usize {
        self.bricks.len() - self.needed().len()
    }

    /// Bricks that would topple the tower if removed.
    fn needed(&self) -> HashSet<BrickId> {
        let mut out = HashSet::new();

        for b in self.bricks.values() {
            let bricks_below: HashSet<_> = b
                .points
                .iter()
                .filter_map(|&p| self.space.get(&(p + DOWN)))
                .filter(|&&id| id != b.id)
                .collect();

            if bricks_below.len() == 1 {
                let needed = *bricks_below.into_iter().next().unwrap();
                out.insert(needed);
            }
        }

        out
    }
}
