advent_of_code::solution!(13);
use std::str::FromStr;

use advent_of_code::helpers::*;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|s| s.parse::<Pattern>().unwrap())
            .map(|p| p.fold_location())
            .map(|loc| match loc {
                FoldLocation::Row(r) => 100 * r,
                FoldLocation::Column(c) => c,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|s| s.parse::<Pattern>().unwrap())
            .map(|p| p.smudge_fold_location())
            .map(|loc| match loc {
                FoldLocation::Row(r) => 100 * r,
                FoldLocation::Column(c) => c,
            })
            .sum(),
    )
}

impl Pattern {
    fn smudge_fold_location(mut self) -> FoldLocation {
        let original = self
            .fold_location_new(None)
            .expect("original has a fold location");

        for y in 0..self.0.height() {
            for x in 0..self.0.width() {
                let p: Point = (x, y).into();
                self.0[p].invert();

                let new = self.fold_location_new(Some(original));

                self.0[p].invert();

                if new.is_some_and(|new| new != original) {
                    return new.unwrap();
                }
            }
        }

        panic!("no fold location found")
    }

    fn fold_location(&self) -> FoldLocation {
        self.fold_location_new(None).unwrap()
    }

    fn fold_location_new(&self, cant_be: Option<FoldLocation>) -> Option<FoldLocation> {
        let locs = (0..self.0.width())
            .map(FoldLocation::Column)
            .chain((0..self.0.height()).map(FoldLocation::Row));

        for l in locs {
            let is_eq = match l {
                FoldLocation::Row(r) => {
                    if r == 0 {
                        continue;
                    }
                    let len = (r).min(self.0.height() - r);
                    let lhs = ((r - len)..r).map(|i| self.0.row(i)).collect_vec();
                    let rhs = (r..(r + len)).rev().map(|i| self.0.row(i)).collect_vec();
                    assert_eq!(lhs.len(), rhs.len());

                    lhs == rhs
                }
                FoldLocation::Column(c) => {
                    if c == 0 {
                        continue;
                    }
                    let len = (c).min(self.0.width() - c);
                    let lhs = ((c - len)..c).map(|i| self.0.col(i)).collect_vec();
                    let rhs = (c..(c + len)).rev().map(|i| self.0.col(i)).collect_vec();
                    assert_eq!(lhs.len(), rhs.len());

                    lhs == rhs
                }
            };

            if is_eq && !cant_be.is_some_and(|cant_be| cant_be == l) {
                return Some(l);
            }
        }

        None
    }
}

impl Tile {
    fn invert(&mut self) {
        *self = match self {
            Tile::Ash => Tile::Rock,
            Tile::Rock => Tile::Ash,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Clone, Deref, DerefMut)]
struct Pattern(Grid<Tile>);

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Rock,
            '.' => Tile::Ash,
            _ => panic!("Invalid tile"),
        }
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_chars(s);
        Ok(Pattern(grid))
    }
}

/// if the fold is between column 5 and 6, then it would be Column(5)
/// and the same for Row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FoldLocation {
    Row(usize),
    Column(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#,
        FoldLocation::Column(5)
    )]
    #[case(
        r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#,
        FoldLocation::Row(4)
    )]
    fn test_fold_location(#[case] pattern: Pattern, #[case] loc: FoldLocation) {
        assert_eq!(pattern.fold_location(), loc);
    }

    #[rstest]
    #[case(
        r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#,
        FoldLocation::Row(3)
    )]
    #[case(
        r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#,
        FoldLocation::Row(1)
    )]
    fn test_smudged_location(#[case] pattern: Pattern, #[case] loc: FoldLocation) {
        assert_eq!(pattern.smudge_fold_location(), loc);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
