use std::io;

use anyhow::{bail, Result};
use itertools::Itertools;

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut input = read_input()?;
    input.roll_north();
    let ans = input.north_load();
    dbg!(ans);
    Ok(())
}

// Ok, probably too slow. Looks like it'd take at least a day to run.
fn main() -> Result<()> {
    let mut input = read_input()?;
    for i in 0..10_usize.pow(9) {
        if i % 10_usize.pow(4) == 0 {
            dbg!(i);
        }
        input.spin_cycle();
    }
    let ans = input.north_load();
    dbg!(ans);
    Ok(())
}

fn read_input() -> Result<Input> {
    let grid = io::stdin().lines().map(parse_line).try_collect()?;
    Ok(Input { grid })
}

fn parse_line(line: io::Result<String>) -> Result<Vec<Tile>> {
    line?.chars().map(parse_tile).collect()
}

fn parse_tile(c: char) -> Result<Tile> {
    let out = match c {
        '.' => Tile::Empty,
        'O' => Tile::Rock,
        '#' => Tile::Obstacle,
        _ => bail!("invalid tile symbol: {c:?}"),
    };
    Ok(out)
}

struct Input {
    grid: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Obstacle,
}

impl Input {
    fn spin_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn roll_north(&mut self) {
        let (_, ncols) = self.dims();
        for col in 0..ncols {
            let groups = self.first_pass(col);
            self.second_pass(col, &groups);
        }
    }

    fn roll_south(&mut self) {
        self.flip_north_south();
        self.roll_north();
        self.flip_north_south();
    }

    fn roll_west(&mut self) {
        self.transpose();
        self.roll_north();
        self.transpose();
    }

    fn roll_east(&mut self) {
        self.transpose();
        self.roll_south(); // (south!)
        self.transpose();
    }

    fn north_load(&self) -> usize {
        let (nrows, ncols) = self.dims();

        let mut total = 0;
        for i in 0..nrows {
            for j in 0..ncols {
                if self.grid[i][j] == Tile::Rock {
                    let load = nrows - i;
                    total += load;
                }
            }
        }
        total
    }

    /// Pick up all the rocks.
    fn first_pass(&mut self, col: usize) -> Vec<usize> {
        let (nrows, _) = self.dims();

        let mut groups = vec![];
        let mut curr_group = 0;
        for row in 0..=nrows {
            // Edge-case: the end of the column.
            let mut terminator = Tile::Obstacle;

            let tile = if row == nrows {
                &mut terminator
            } else {
                &mut self.grid[row][col]
            };

            match tile {
                Tile::Obstacle => {
                    groups.push(curr_group);
                    curr_group = 0;
                }
                Tile::Rock => {
                    *tile = Tile::Empty;
                    curr_group += 1;
                }
                Tile::Empty => (),
            }
        }
        groups
    }

    /// Re-distribute them.
    fn second_pass(&mut self, col: usize, groups: &[usize]) {
        let (nrows, _) = self.dims();

        let mut groups = groups.iter().copied();
        let mut curr_group = groups.next().expect("empty groups");
        for row in 0..nrows {
            let tile = &mut self.grid[row][col];
            match *tile {
                Tile::Empty => {
                    if curr_group != 0 {
                        *tile = Tile::Rock;
                        curr_group -= 1;
                    }
                }
                Tile::Obstacle => {
                    assert_eq!(curr_group, 0);
                    curr_group = groups.next().expect("ran out of groups");
                }
                Tile::Rock => {
                    dbg!(row, col);
                    panic!("didn't pick up all the rocks");
                }
            }
        }

        assert!(groups.next().is_none(), "too many groups");
    }

    fn flip_north_south(&mut self) {
        let (nrows, ncols) = self.dims();
        for j in 0..ncols {
            for i in 0..nrows / 2 {
                let tmp = self.grid[i][j];
                self.grid[i][j] = self.grid[nrows - 1 - i][j];
                self.grid[nrows - 1 - i][j] = tmp;
            }
        }
    }

    fn transpose(&mut self) {
        let (nrows, ncols) = self.dims();

        let mut out = vec![vec![Tile::Empty; nrows]; ncols]; // note the swap !
        for i in 0..nrows {
            for j in 0..ncols {
                out[j][i] = self.grid[i][j];
            }
        }
        self.grid = out;
    }

    fn dims(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        (nrows, ncols)
    }
}
