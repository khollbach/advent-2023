use anyhow::Result;

const INPUTS: [(u32, u32); 4] = [
    (50, 242),
    (74, 1017),
    (86, 1691),
    (85, 1252),
];

fn main() -> Result<()> {
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
