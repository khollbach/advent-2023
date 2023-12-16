use std::io;

use anyhow::{bail, ensure, Result};
use itertools::Itertools;

// Hrmmmm. This doesn't even seem much faster.
fn main() -> Result<()> {
    let mut grid = Grid::new(read_input()?)?;
    for i in 0..10_usize.pow(9) {
        if i % 10_usize.pow(4) == 0 {
            dbg!(i);
        }
        grid.spin_cycle();
    }
    let ans = grid.north_load();
    dbg!(ans);
    Ok(())
}

type Input = Vec<Vec<Tile>>;

fn read_input() -> Result<Input> {
    io::stdin().lines().map(parse_line).try_collect()
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

struct Grid {
    // grid: Input,
    grid: [[Tile; 100]; 100],
    transposed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Obstacle,
}

impl Grid {
    fn new(input: Input) -> Result<Self> {
        ensure!(input.len() == 100);
        ensure!(input.iter().all(|row| row.len() == 100));

        let mut grid = [[Tile::Empty; 100]; 100];
        for i in 0..100 {
            for j in 0..100 {
                grid[i][j] = input[i][j];
            }
        }

        Ok(Self {
            grid,
            transposed: false,
        })
    }

    fn spin_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn roll_east(&mut self) {
        let (nrows, _) = self.dims();
        for row in 0..nrows {
            let groups = self.first_pass(row);
            self.second_pass(row, &groups);
        }
    }

    fn roll_west(&mut self) {
        self.flip_east_west();
        self.roll_east();
        self.flip_east_west();
    }

    fn roll_south(&mut self) {
        self.transpose();
        self.roll_east();
        self.transpose();
    }

    fn roll_north(&mut self) {
        // Flipping east/west before transposing is the same as flipping
        // north/south after transposing.
        self.flip_east_west();
        self.transpose();

        self.roll_east();

        self.transpose();
        self.flip_east_west();
    }

    fn north_load(&self) -> usize {
        let (nrows, ncols) = self.dims();

        let mut total = 0;
        for i in 0..nrows {
            for j in 0..ncols {
                if self.get(i, j) == Tile::Rock {
                    let load = nrows - i;
                    total += load;
                }
            }
        }
        total
    }

    /// Pick up all the rocks.
    fn first_pass(&mut self, row: usize) -> Vec<usize> {
        let (_, ncols) = self.dims();

        let mut groups = vec![];
        let mut curr_group = 0;
        for col in 0..=ncols {
            // Edge-case: the end of the row.
            let mut terminator = Tile::Obstacle;

            let tile = if col == ncols {
                &mut terminator
            } else {
                self.get_mut(row, col)
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
    fn second_pass(&mut self, row: usize, groups: &[usize]) {
        let (_, ncols) = self.dims();

        let mut groups = groups.iter().copied();
        let mut curr_group = groups.next().expect("empty groups");
        for col in 0..ncols {
            let tile = self.get_mut(row, col);
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

    fn flip_east_west(&mut self) {
        assert!(!self.transposed);

        let (nrows, ncols) = self.dims();
        for i in 0..nrows {
            for j in 0..ncols / 2 {
                let tmp = self.grid[i][j];
                self.grid[i][j] = self.grid[i][ncols - 1 - j];
                self.grid[i][ncols - 1 - j] = tmp;
            }
        }
    }

    fn transpose(&mut self) {
        self.transposed ^= true;
    }

    fn get(&self, i: usize, j: usize) -> Tile {
        if self.transposed {
            self.grid[j][i]
        } else {
            self.grid[i][j]
        }
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Tile {
        if self.transposed {
            &mut self.grid[j][i]
        } else {
            &mut self.grid[i][j]
        }
    }

    fn dims(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        (nrows, ncols)
    }
}
