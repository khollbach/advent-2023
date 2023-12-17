use std::{array, io, str::FromStr};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut sum = 0;
    for s in read_input_part_1()? {
        sum += HASH(&s);
    }
    dbg!(sum);
    Ok(())
}

fn main() -> Result<()> {
    let mut map = HASHMAP::new();
    for cmd in read_input_part_2()? {
        match cmd {
            Command::Insert {
                label,
                focal_length,
            } => map.insert(label, focal_length),
            Command::Remove { label } => map.remove(&label),
        }
    }

    let mut total = 0;
    for (bucket_idx, bucket) in (1..).zip(map.buckets) {
        for (lens_idx, (_, focal_length)) in (1..).zip(bucket) {
            let focusing_power = bucket_idx * lens_idx * focal_length;
            total += focusing_power;
        }
    }
    dbg!(total);

    Ok(())
}

fn read_input_part_1() -> Result<Vec<String>> {
    let mut lines = io::stdin().lines();
    let l = lines.next().context("empty")??;
    ensure!(lines.next().is_none(), "too many lines");
    Ok(l.split(',').map(String::from).collect())
}

fn read_input_part_2() -> Result<Vec<Command>> {
    let mut lines = io::stdin().lines();
    let l = lines.next().context("empty")??;
    ensure!(lines.next().is_none(), "too many lines");
    l.split(',').map(str::parse).try_collect()
}

enum Command {
    Insert { label: String, focal_length: u32 },
    Remove { label: String },
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.ends_with('-') {
            let label = s[..s.len() - 1].to_owned();
            return Ok(Self::Remove { label });
        }

        let (label, focal_length) = s.split_once('=').context("equals")?;
        let label = label.to_owned();
        let focal_length = focal_length.parse()?;
        Ok(Self::Insert {
            label,
            focal_length,
        })
    }
}

#[allow(non_snake_case)]
fn HASH(s: &str) -> u32 {
    let mut curr_val = 0;
    for c in s.chars() {
        curr_val += c as u32;
        curr_val *= 17;
        curr_val %= 256;
    }
    curr_val
}

struct HASHMAP {
    buckets: [Bucket; 256],
}

type Bucket = Vec<(String, u32)>;

impl HASHMAP {
    fn new() -> Self {
        Self {
            buckets: array::from_fn(|_| vec![]),
        }
    }

    fn insert(&mut self, label: String, focal_length: u32) {
        let bucket = &mut self.buckets[HASH(&label) as usize];
        let exists = bucket_contains_key(bucket, &label);
        let kv_pair = (label, focal_length);
        if let Some(idx) = exists {
            bucket[idx] = kv_pair;
        } else {
            bucket.push(kv_pair);
        }
    }

    fn remove(&mut self, label: &str) {
        let bucket = &mut self.buckets[HASH(label) as usize];
        if let Some(idx) = bucket_contains_key(bucket, label) {
            bucket.remove(idx);
        }
    }
}

fn bucket_contains_key(bucket: &Bucket, label: &str) -> Option<usize> {
    bucket
        .iter()
        .enumerate()
        .find_map(|(i, (l, _))| if l == label { Some(i) } else { None })
}
