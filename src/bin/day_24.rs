use std::{
    io,
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

fn main() -> Result<()> {
    let rays = read_input()?;

    for x in -100..=100 {
        for y in -100..=100 {
            for z in -100..=100 {
                let d = Point::from((x, y, z));
                if is_winning(d, &rays) {
                    dbg!(d);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn is_winning(normal: Point, lines: &[Ray]) -> bool {
    let intersect = |a: Ray, b: Ray| {
        let a = a.project_onto_plane(normal);
        let b = b.project_onto_plane(normal);
        intersection(a, b).unwrap_or_default()
    };

    let n = lines.len();
    let expected = intersect(lines[0], lines[1]);
    for i in 0..n {
        for j in i + 1..n {
            let actual = intersect(lines[i], lines[j]);
            let delta = (expected - actual).norm();
            if delta > 10. {
                return false;
            }
        }
    }
    true
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
    ///
    /// The normal is orthogonal to the plane, but might not be normalized.
    fn project_onto_plane(self, normal: Self) -> Self {
        let normal = normal.normalize();
        self - normal.scale(self.dot(normal))
    }

    #[must_use]
    fn normalize(self) -> Self {
        self.scale(1. / self.norm())
    }
}

// But now how to intersect the projected lines, which still live in 3-space?
// Could we instead project them in such a way that they produce Point2's ?
// That'd let us use our existing code for this step.

fn intersection(a: Ray, b: Ray) -> Option<Point> {
    todo!()
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

#[derive(Debug, Clone, Copy)]
struct Ray {
    start: Point,
    direction: Point,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// fn parallel(a: Point, b: Point) -> bool {
//     assert_ne!(a.x, 0.);
//     assert_ne!(a.y, 0.);
//     assert_ne!(a.z, 0.);

//     assert_ne!(b.x, 0.);
//     assert_ne!(b.y, 0.);
//     assert_ne!(b.z, 0.);

//     // Scale up both, so their x components are equal.
//     let (a, b) = (a.scale(b.x), b.scale(a.x));

//     // todo: do we want to handle this case?
//     assert_ne!(a.scale(-1.), b);

//     a == b
// }

// fn xy_parallel(a: Point, b: Point) -> bool {
//     a.y * b.x == b.y * a.x
// }

// impl Point {
//     fn is_in_test_area(self) -> bool {
//         let (low, high) = (2e14, 4e14);
//         let x = low <= self.x && self.x <= high;
//         let y = low <= self.y && self.y <= high;
//         x && y
//     }
// }

// #[allow(dead_code)]
// fn main_() -> Result<()> {
//     let rays = read_input()?;

//     let n = rays.len();
//     for i in 0..n {
//         let p = Point {
//             x: 1.,
//             y: 1.,
//             z: 0.,
//         };
//         if xy_parallel(rays[i].direction, p) {
//             dbg!(rays[i]);
//         }
//     }

//     let avg_x: f64 = rays.iter().map(|r| r.direction.x).sum::<f64>() / rays.len() as f64;
//     let avg_y: f64 = rays.iter().map(|r| r.direction.y).sum::<f64>() / rays.len() as f64;
//     let avg_z: f64 = rays.iter().map(|r| r.direction.z).sum::<f64>() / rays.len() as f64;
//     dbg!(avg_x, avg_y, avg_z);

//     let avg_x: f64 = rays.iter().map(|r| r.start.x).sum::<f64>() / rays.len() as f64;
//     let avg_y: f64 = rays.iter().map(|r| r.start.y).sum::<f64>() / rays.len() as f64;
//     let avg_z: f64 = rays.iter().map(|r| r.start.z).sum::<f64>() / rays.len() as f64;
//     dbg!(avg_x, avg_y, avg_z);

//     Ok(())
// }

// ---

// impl Point {
//     #[must_use]
//     fn scale(self, a: f64) -> Self {
//         Self {
//             x: self.x * a,
//             y: self.y * a,
//             z: self.z * a,
//         }
//     }
// }

impl Point {
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

// fn intersection(a: Ray, b: Ray) -> Option<Intersection> {
//     if parallel(a.direction, b.direction) {
//         assert_ne!(a.start, b.start);
//         return None;
//     }

//     assert_ne!(a.direction.x, 0.);
//     let ratio = a.direction.y / a.direction.x;
//     let lhs = b.start.y - a.start.y + ratio * (a.start.x - b.start.x);
//     let rhs = ratio * b.direction.x - b.direction.y;
//     let time_b = lhs / rhs;
//     let ans = b.start + b.direction.scale(time_b);

//     let time_a = (b.start.x - a.start.x + time_b * b.direction.x) / a.direction.x;
//     let ans2 = a.start + a.direction.scale(time_a);
//     assert_eq!(ans.is_in_test_area(), ans2.is_in_test_area());
//     if ans.is_in_test_area() {
//         let delta = (ans - ans2).norm();
//         assert!(
//             delta < 10.,
//             "answers differ by a lot: {ans:?} vs {ans2:?} ({delta})"
//         );
//     }

//     Some(Intersection {
//         time_a,
//         time_b,
//         xy_position: ans,
//     })
// }

// struct Intersection {
//     time_a: f64,
//     time_b: f64,
//     xy_position: Point,
// }

// fn main() -> Result<()> {
//     let rays = read_input()?;

//     let mut count = 0;
//     let n = rays.len();
//     for i in 0..n {
//         for j in i + 1..n {
//             let Some(int) = intersection(rays[i], rays[j]) else {
//                 continue;
//             };
//             if int.time_a >= 0. && int.time_b >= 0. && int.xy_position.is_in_test_area() {
//                 count += 1;
//             }
//         }
//     }
//     dbg!(count);

//     Ok(())
// }
