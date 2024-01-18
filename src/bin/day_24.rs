use std::{
    fmt, io,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{Context, Result};
use itertools::Itertools;

impl From<(i32, i32, i32)> for Point {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }
}

// sample input answer: -3, 1, 2
// (3,-1,-2 also works)
fn main() -> Result<()> {
    let lines = read_input()?;

    // This direction and norm seem like a pretty good fit for the input data.
    // I'm not certain this is the exact right direction, but it'll be a good
    // starting guess if we end up doing a proper gradient descent.
    let (x, y, z) = (45., 306., 76.);
    let winning_dir = Point { x, y, z }.scale(1.);
    dbg!(winning_dir);
    dbg!(winning_dir.norm());
    dbg!(winning_dir.normalize());

    let points = projected_intersections(winning_dir, &lines);
    let direction_score = closeness_score(&points);
    dbg!(direction_score.log10());

    let n = points.len();
    let x = points.iter().map(|p| p.x).sum::<f64>() / n as f64;
    let y = points.iter().map(|p| p.y).sum::<f64>() / n as f64;
    let avg_intersection = Point2 { x, y };
    dbg!(avg_intersection);

    // This attempts to guess the starting point, based on the direction and norm.
    let mut guesses = vec![];
    let normal = winning_dir.normalize();
    for l in lines {
        let proj = l.project_onto_plane(normal).project_xy();
        let t = (avg_intersection - proj.start).norm() / proj.direction.norm();
        let target = l.start + l.direction.scale(t);
        let guess = target - winning_dir.scale(t);
        guesses.push(guess);
    }
    let n = guesses.len();
    let x = guesses.iter().map(|p| p.x).sum::<f64>() / n as f64;
    let y = guesses.iter().map(|p| p.y).sum::<f64>() / n as f64;
    let z = guesses.iter().map(|p| p.z).sum::<f64>() / n as f64;
    let avg_guess = Point { x, y, z };
    dbg!(avg_guess);

    let x = guesses
        .iter()
        .map(|p| (p.x - avg_guess.x))
        .max_by_key(|&f| f as u64)
        .unwrap()
        / avg_guess.x;
    let y = guesses
        .iter()
        .map(|p| (p.y - avg_guess.y))
        .max_by_key(|&f| f as u64)
        .unwrap()
        / avg_guess.y;
    let z = guesses
        .iter()
        .map(|p| (p.z - avg_guess.z))
        .max_by_key(|&f| f as u64)
        .unwrap()
        / avg_guess.z;
    let max_diffs = Point { x, y, z };
    dbg!(max_diffs);

    // Ok so this gets us pretty close, but we still have ~3% error in the y coord.
    // (x and z coords are less than .5% error)
    // What do?

    // Guesses made on the website:
    // 233_740_160_691_462 + 100_345_639_785_493 + 230_473_854_139_001 = 564_559_654_615_956
    // (too low)
    // 567_000_000_000_000 -- added 3% to the y coord
    // (too high)
    // 565_000_000_000_000 -- since it's a nice round number
    // (too low)

    // Ok great, so we've narrowed it down to a range of 3 trillion numbers...
    // Now what? ... It's not good enough to be within 1% of the actual answer;
    // we need to find *exact integer values*... So maybe something fundamental
    // needs to change about the approach?

    // Two vague ways forward that I can think of:
    // 1. sharpen the manual trial-and-error approach into a proper "gradient descent"
    //      (this must converge to arbitrary precision for it to work!)
    //      I'm a little dubious of the winning-direction being possible to find exaclty
    //      using the current method, since I haven't been abe to get the average
    //      distance between projected intersections down lower than ~100M = 10^8.
    //      (Which is actually a very small error value -- something like .000_01%)
    //      But honestly, then again, why should I be able to get better than that by
    //      hand ? And if the current approach is able to do that with so few (human)
    //      iterations, then maybe that says something about its strength? ... hmm...
    //
    //      Ok, yeah. I think this is a good avenue to pursue further. If nothing else,
    //      it'll force me to learn what gradient descent is. And that seems like it'll
    //      be fun, and worth it.
    //
    // 2. think of something new entirely...
    //      ...but what?
    //      My vague intuition is that we're currently not doing much useful with the
    //      time axis. Maybe we could take advantage of that part of the input shape
    //      somehow?

    Ok(())
}

fn intersections_2d(lines: &[Ray2]) -> Vec<Point2> {
    let n = lines.len();
    let mut out = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if let Some(int) = intersection2(lines[i], lines[j]) {
                out.push(int.xy_position);
            }
        }
    }
    out
}

// But now how to intersect the projected lines, which still live in 3-space?
// Could we instead project them in such a way that they produce Point2's ?
// That'd let us use our existing code for this step.
//
// Idea: project them again, onto the xy plane. It'll skew them, but shouldn't
// change whether there's a single intersection point or not.

fn projected_intersections(direction: Point, lines: &[Ray]) -> Vec<Point2> {
    let normal = direction.normalize();
    let project = |a: Ray| -> Ray2 { a.project_onto_plane(normal).project_xy() };

    let lines_2d: Vec<_> = lines.iter().copied().map(project).collect();
    let ints = intersections_2d(&lines_2d);

    // Reasonableness check. If my mental model of the input is correct, then
    // very few lines should be parallel. So there should be many intersections.
    let n = lines.len();
    assert!(n >= 5);
    let max_intersections = n * (n - 1) / 2;
    if ints.len() < max_intersections / 4 {
        // This probably means we projected onto the yz plane (by choosing the x
        // axis as a normal), and then onto the xy plane. Since those two planes
        // are orthogonal (their projection onto each other is only a line), we
        // can't use them to check if this normal is winning.
        //
        // We'll ignore this case, since we're assuming the answer isn't one of
        // the axes. (I suppose we could check this just to be sure? todo...)
        // let tmp = normal == Point::from((1, 0, 0))
        //         || normal == Point::from((-1, 0, 0))
        //         || normal == Point::from((0, 1, 0))
        //         || normal == Point::from((0, -1, 0));
        // if !tmp {
        //     dbg!(direction, lines, lines_2d, ints.len(), max_intersections);
        //     dbg!(ints);
        // }
        // assert!(
        //     normal == Point::from((1, 0, 0))
        //         || normal == Point::from((-1, 0, 0))
        //         || normal == Point::from((0, 1, 0))
        //         || normal == Point::from((0, -1, 0))
        // );
        // return (false, ints);

        // dummy values to get a bad closeness score
        return vec![Point2 { x: 0., y: 0. }, Point2 { x: 1e9, y: 1e9 }];
    }

    ints

    // // Do they all intersect in the same spot?
    // let expected = ints[0];
    // for &actual in &ints {
    //     let delta = (expected - actual).norm();
    //     if !(delta < 0.1) {
    //         return (false, ints);
    //     }
    // }
    // (true, ints)
}

/// Lower is better.
fn closeness_score(points: &[Point2]) -> f64 {
    let n = points.len();
    let x = points.iter().map(|p| p.x).sum::<f64>() / n as f64;
    let y = points.iter().map(|p| p.y).sum::<f64>() / n as f64;
    let avg = Point2 { x, y };

    let mut score = 0.;
    for p in points {
        score += (p.x - avg.x).abs();
        score += (p.y - avg.y).abs();
    }
    score / n_choose_2(n) as f64
}

fn n_choose_2(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    n * (n - 1) / 2
}

impl Ray {
    fn project_onto_plane(self, normal: Point) -> Self {
        Self {
            start: self.start.project_onto_plane(normal),
            direction: self.direction.project_onto_plane(normal),
        }
    }
}

impl Point {
    /// The plane goes through (0,0,0).
    fn project_onto_plane(self, normal: Self) -> Self {
        assert!(is_close(normal.norm(), 1., 0.1));
        let out = self - normal.scale(self.dot(normal));
        // assert!(out.dot(normal) < 0.1);
        out
    }

    #[must_use]
    fn normalize(self) -> Self {
        let out = self.scale(1. / self.norm());
        assert!(is_close(out.norm(), 1., 0.1));
        out
    }
}

fn is_close(a: f64, b: f64, eps: f64) -> bool {
    assert!(eps >= 0.);
    (a - b).abs() <= eps
}

// ---

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
        let (x, y, z) = s
            .split(", ")
            .map(|word| word.parse().unwrap())
            .collect_tuple()
            .context("triple")?;
        Ok(Self { x, y, z })
    }
}

#[derive(Clone, Copy)]
struct Ray {
    start: Point,
    direction: Point,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Ray {
    fn project_xy(self) -> Ray2 {
        Ray2 {
            start: self.start.project_xy(),
            direction: self.direction.project_xy(),
        }
    }
}

impl Point {
    // todo: what if, for now, we tried projecting onto the xz plane instead?
    // (doesn't seem to make much of a difference.)
    fn project_xy(self) -> Point2 {
        Point2 {
            x: self.x,
            y: self.z,
        }
    }
}

// ---

impl Point {
    fn dot(self, other: Self) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        x + y + z
    }

    #[must_use]
    fn scale(self, a: f64) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    #[must_use]
    fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
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

// ---

#[derive(Clone, Copy)]
struct Ray2 {
    start: Point2,
    direction: Point2,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Point2 {
    x: f64,
    y: f64,
}

/// Return None if the lines are parallel (meaning they have either 0 or
/// infinitely many intersections).
fn intersection2(a: Ray2, b: Ray2) -> Option<Intersection> {
    if a.direction.norm() == 0. || b.direction.norm() == 0. {
        // For part 2, this must mean we projected onto a plane whose normal is
        // collinear with an input ray's direction. It's ok for us to ignore
        // that ray, since the other rays will still interact, and give us
        // enough information to tell if this projection is winning.
        return None;
    }
    if xy_parallel(a.direction, b.direction) {
        return None;
    }

    // Special case: a is a vertical line.
    if a.direction.x == 0. {
        let dx = a.start.x - b.start.x;
        let b_slope = b.direction.y / b.direction.x;
        let ans = Point2 {
            x: a.start.x,
            y: b.start.x + b_slope * dx,
        };

        return Some(Intersection {
            time_a: (ans - a.start).norm() / a.direction.norm(),
            time_b: (ans - b.start).norm() / b.direction.norm(),
            xy_position: ans,
        });
    }

    let ratio = a.direction.y / a.direction.x;
    let lhs = b.start.y - a.start.y + ratio * (a.start.x - b.start.x);
    let rhs = ratio * b.direction.x - b.direction.y;
    let time_b = lhs / rhs;
    let ans = b.start + b.direction.scale(time_b);

    let time_a = (b.start.x - a.start.x + time_b * b.direction.x) / a.direction.x;
    // let ans2 = a.start + a.direction.scale(time_a);
    // assert_eq!(ans.is_in_test_area(), ans2.is_in_test_area());
    // if ans.is_in_test_area() {
    //     let delta = (ans - ans2).norm();
    //     assert!(
    //         delta < 10.,
    //         "answers differ by a lot: {ans:?} vs {ans2:?} ({delta})"
    //     );
    // }

    Some(Intersection {
        time_a,
        time_b,
        xy_position: ans,
    })
}

struct Intersection {
    time_a: f64,
    time_b: f64,
    xy_position: Point2,
}

fn xy_parallel(a: Point2, b: Point2) -> bool {
    assert!(a.norm() > 0., "{a:?}");
    assert!(b.norm() > 0., "{b:?}");
    let x_zero = a.x == 0. && b.x == 0.; // (y axis)
    let y_zero = a.y == 0. && b.y == 0.; // (x axis)
    let no_zeros = ![a.x, a.y, b.x, b.y].contains(&0.);
    // let same_slope = a.y * b.x == b.y * a.x;
    let same_slope = is_close(a.y * b.x, b.y * a.x, 1e-9);
    x_zero || y_zero || no_zeros && same_slope
}

impl Point2 {
    fn is_in_test_area(self) -> bool {
        let (low, high) = (2e14, 4e14);
        let x = low <= self.x && self.x <= high;
        let y = low <= self.y && self.y <= high;
        x && y
    }

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

impl Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let rays = read_input()?;
    let rays: Vec<_> = rays.into_iter().map(Ray::project_xy).collect();

    let mut count = 0;
    let n = rays.len();
    for i in 0..n {
        for j in i + 1..n {
            let Some(int) = intersection2(rays[i], rays[j]) else {
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

struct FloatDisplay(f64);

impl fmt::Display for FloatDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        if x.fract() == 0. {
            write!(f, "{x}")
        } else {
            write!(f, "{x:.2}")
        }
        // write!(f, "{x}")
    }
}

fn fd(x: f64) -> FloatDisplay {
    FloatDisplay(x)
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, y, z } = *self;
        write!(f, "{}, {}, {}", fd(x), fd(y), fd(z))
    }
}

impl fmt::Debug for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, y } = *self;
        write!(f, "{}, {}", fd(x), fd(y))
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { start, direction } = self;
        write!(f, "{start:?} @ {direction:?}")
    }
}

impl fmt::Debug for Ray2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { start, direction } = self;
        write!(f, "{start:?} @ {direction:?}")
    }
}
