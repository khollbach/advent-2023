mod grid;
mod point;
mod walls;

use self::{
    grid::{Grid, GridCoord},
    point::Point,
    walls::Walls,
};
use crate::day_18::{input::Command, part_2::grid::Tile};

use super::input::Dir;

pub fn solve(commands: &[Command]) -> u64 {
    let start = Point { x: 0, y: 0 };
    let points = points(commands, start);
    let grid = Grid::new(&points);
    let trench = trench(&grid, commands, start);
    let walls = Walls::new(&grid, &trench);

    // Guess-and-check a good starting point.
    let start = Tile {
        bottom_left: grid.find(start).unwrap() + Dir::Down,
    };

    let tiles = walls.enclosed_tiles(start);
    let tile_area: u64 = tiles.into_iter().map(|t| grid.area(t)).sum();
    let perimeter: u32 = commands.iter().map(|cmd| cmd.distance).sum();

    // I didn't check the details, but this seems to work.
    tile_area + perimeter as u64 / 2 + 1
}

fn points(commands: &[Command], start: Point) -> Vec<Point> {
    let mut out = vec![];

    let mut curr = start;
    for cmd in commands {
        curr += Point::from(cmd.direction) * cmd.distance as i32;
        out.push(curr);
    }
    assert_eq!(curr, start, "not a loop");

    out
}

fn trench(grid: &Grid, commands: &[Command], start: Point) -> Vec<GridCoord> {
    let mut out = vec![];

    let mut curr = start;
    for cmd in commands {
        // This is naive and slow, but maybe it's fine?
        // The total perimeter is only ~10^9.
        for _ in 0..cmd.distance {
            curr += cmd.direction.into();
            if let Some(grid_coord) = grid.find(curr) {
                out.push(grid_coord);
            }
        }
    }
    assert_eq!(curr, start, "not a loop");

    out
}
