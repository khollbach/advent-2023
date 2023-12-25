use std::{
    io,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{Context, Result};
use itertools::Itertools;

fn read_input() -> Result<Vec<Ray>> {
    let mut out = vec![];
    for l in io::stdin().lines() {
        let l = l?;
        let (pos, dirn) = l.split_once(" @ ").context("@")?;
        out.push(Ray {
            start: pos.parse()?,
            direction: dirn.parse()?,
        });
    }
    Ok(out)
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, _z) = s
            .split(", ")
            .map(|word| word.parse().unwrap())
            .collect_tuple()
            .context("triple")?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    start: Point,
    direction: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn parallel(a: Point, b: Point) -> bool {
    assert_ne!(a.x, 0.);
    assert_ne!(b.x, 0.);
    a.y * b.x == b.y * a.x
}

fn intersection(a: Ray, b: Ray) -> Option<Intersection> {
    if parallel(a.direction, b.direction) {
        assert_ne!(a.start, b.start);
        return None;
    }

    assert_ne!(a.direction.x, 0.);
    let ratio = a.direction.y / a.direction.x;
    let lhs = b.start.y - a.start.y + ratio * (a.start.x - b.start.x);
    let rhs = ratio * b.direction.x - b.direction.y;
    let time_b = lhs / rhs;
    let ans = b.start + b.direction.scale(time_b);

    let time_a = (b.start.x - a.start.x + time_b * b.direction.x) / a.direction.x;
    let ans2 = a.start + a.direction.scale(time_a);
    assert_eq!(ans.is_in_test_area(), ans2.is_in_test_area());
    if ans.is_in_test_area() {
        let delta = (ans - ans2).norm();
        assert!(
            delta < 10.,
            "answers differ by a lot: {ans:?} vs {ans2:?} ({delta})"
        );
    }

    Some(Intersection {
        time_a,
        time_b,
        xy_position: ans,
    })
}

struct Intersection {
    time_a: f64,
    time_b: f64,
    xy_position: Point,
}

impl Point {
    fn is_in_test_area(self) -> bool {
        let (low, high) = (2e14, 4e14);
        let x = low <= self.x && self.x <= high;
        let y = low <= self.y && self.y <= high;
        x && y
    }
}

fn main() -> Result<()> {
    let rays = read_input()?;

    let mut count = 0;
    let n = rays.len();
    for i in 0..n {
        for j in i + 1..n {
            let Some(int) = intersection(rays[i], rays[j]) else {
                continue;
            };
            if int.time_a >= 0. && int.time_b >= 0. && int.xy_position.is_in_test_area() {
                count += 1;
            }
        }
    }
    dbg!(count);

    Ok(())
}

// ---

impl Point {
    #[must_use]
    fn scale(self, a: f64) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
        }
    }

    #[must_use]
    fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
