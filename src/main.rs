use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let a = line.chars().nth(first).unwrap();
        let b = line.chars().nth(last).unwrap();
        let ab: String = [a, b].into_iter().collect();
        let n: u32 = ab.parse().unwrap();
        sum += n;
    }
    dbg!(sum);
    Ok(())
}
