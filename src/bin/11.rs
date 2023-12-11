advent_of_code::solution!(11);
use std::fmt::Display;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1_000_000))
}

fn solve(input: &str, multiplier: u64) -> u64 {
    let ((expanded_rows, expanded_cols), galaxies) = parse(input);

    let mut sum = 0u64;
    for (start, end) in galaxies.iter().tuple_combinations() {
        sum += start.manhattan_distance(end) as u64;

        // start by iterating over the x values between the two galaxies
        let x_crossings = (start.x_between(end))
            // then filter out the ones that aren't in the expanded rows
            .filter(|x| expanded_cols.contains(x))
            // and count it
            .count() as u64;

        // same thing in the y direction
        let y_crossings = start
            .y_between(end)
            .filter(|&y| expanded_rows.contains(&y))
            .count() as u64;

        // already 1 space between the two galaxies, so subtract by 1
        sum += (x_crossings + y_crossings) * (multiplier - 1);
    }

    sum
}

/// (expanded_rows, expanded_cols), galaxies
fn parse(input: &str) -> ((Vec<usize>, Vec<usize>), Vec<Point>) {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let h = grid.height();
    let w = grid.width();

    let expanded_rows = (0..w)
        .map(|i| (i, grid.row(i)))
        .filter(|(_, r)| r.iter().all(|&t| t == Tile::Empty))
        .map(|(i, _)| i)
        .collect_vec();

    let expanded_cols = (0..h)
        .map(|i| (i, grid.col(i)))
        .filter(|(_, c)| c.iter().all(|&&t| t == Tile::Empty))
        .map(|(i, _)| i)
        .collect_vec();

    let galaxies: Vec<Point> = grid
        .flat_iter()
        .filter(|&(t, _)| *t == Tile::Galaxy)
        .map(|(_, p)| p)
        .collect();

    ((expanded_rows, expanded_cols), galaxies)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    Galaxy,
    #[default]
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Galaxy,
            _ => panic!("Unknown tile type: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Galaxy => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);

        assert_eq!(solve(&input, 10), 1030);
        assert_eq!(solve(&input, 100), 8410);
        // i find it very funny that they just didn't give us this number LMAO
        assert_eq!(solve(&input, 1_000_000), 82000210);
    }
}
