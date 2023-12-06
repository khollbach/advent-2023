use anyhow::Result;

const INPUTS: [(u32, u32); 4] = [
    (50, 242),
    (74, 1017),
    (86, 1691),
    (85, 1252),
];

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut total = 1;

    for (time, distance) in INPUTS {
        let mut ans = 0;
        for t in 0..=time {
            if t * (time - t) > distance {
                ans += 1;
            }
        }

        total *= ans;
    }

    dbg!(total);
    Ok(())
}

fn main() -> Result<()> {
    let time = 50_74_86_85;
    let distance = 242_1017_1691_1252;

    let mut ans = 0;
    for t in 0u64..=time {
        if t * (time - t) > distance {
            ans += 1;
        }
    }
    dbg!(ans);

    Ok(())
}
