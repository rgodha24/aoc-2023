advent_of_code::solution!(16);
use advent_of_code::helpers::*;
use std::collections::HashSet;

// this is an anti pattern in normal rust but its fine in AOC bc we're not
// gonna change the definition of these enums.
use Direction::*;
use Tile::*;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<Tile> = Grid::from_chars(input);

    Some(energized(&grid, Point::new(0, 0), Right))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let w = grid.width();
    let h = grid.height();

    let lefts = grid.y_points_at(0).map(|p| (p, Right));
    let rights = grid.y_points_at(w - 1).map(|p| (p, Left));
    let tops = grid.x_points_at(0).map(|p| (p, Down));
    let bottoms = grid.x_points_at(h - 1).map(|p| (p, Up));

    let iter = lefts
        .chain(rights)
        .chain(tops)
        .chain(bottoms)
        .map(|(p, h)| energized(&grid, p, h));

    iter.max()
}

fn energized(tiles: &Grid<Tile>, start_p: Point, start_h: Direction) -> u32 {
    let mut energized: Grid<bool> = tiles.empty_sized();
    // stop infinite loops by keeping track of the splits we've already taken
    // we can only take a split once because splits are the same no matter if we enter from
    // top/bottom (for h-splits) or left/right (for v-splits)
    // we don't count left/right for h-splits or vice versa, so we don't need to worry about that
    let mut taken_splits: HashSet<Point> = HashSet::new();

    f(start_p, start_h, &mut energized, tiles, &mut taken_splits);

    // println!("{}", energized.map(|&b, _| if b { '#' } else { '.' }));

    energized.flat_iter().filter(|(&b, _)| b).count() as u32
}

// can't think of what to call this function but it's a recursive function that
// mutates the energized grid based on its current heading + position
fn f(
    mut p: Point,
    mut heading: Direction,
    energized: &mut Grid<bool>,
    grid: &Grid<Tile>,
    taken_splits: &mut HashSet<Point>,
) {
    loop {
        // stop ourselves if we're off the map
        if !grid.contains_point(p) {
            return;
        }
        let tile = grid[p];
        energized[p] = true;

        match (heading, tile) {
            // split recursively if we haven't already
            (Up | Down, HorizontalSplit) => {
                if taken_splits.insert(p) {
                    f(p, Left, energized, grid, taken_splits);
                    f(p, Right, energized, grid, taken_splits);
                }
                return;
            }
            (Left | Right, VerticalSplit) => {
                if taken_splits.insert(p) {
                    f(p, Up, energized, grid, taken_splits);
                    f(p, Down, energized, grid, taken_splits);
                }
                return;
            }

            // switch directions, but don't split
            (Up, DownMirror) | (Down, UpMirror) => {
                heading = Left;
            }
            (Up, UpMirror) | (Down, DownMirror) => {
                heading = Right;
            }
            (Left, DownMirror) | (Right, UpMirror) => {
                heading = Up;
            }
            (Left, UpMirror) | (Right, DownMirror) => {
                heading = Down;
            }

            // treat as empty space and just continue bc we already energized it
            (_, Empty) | (Up | Down, VerticalSplit) | (Left | Right, HorizontalSplit) => {}
        }

        p += &heading;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// up mirror and down mirror are based on the right side of the slash
enum Tile {
    /// .
    Empty,
    /// |
    VerticalSplit,
    /// -
    HorizontalSplit,
    /// \
    DownMirror,
    /// /
    UpMirror,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '|' => VerticalSplit,
            '-' => HorizontalSplit,
            '\\' => DownMirror,
            '/' => UpMirror,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
