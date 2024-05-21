advent_of_code::solution!(21);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 64))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' | 'S' => Tile::Plot,
            '#' => Tile::Rock,
            _ => panic!("Invalid tile"),
        }
    }
}

fn solve_wrapping(input: &str, steps: usize) -> usize {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let start = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'S')
                .map(move |(x, _)| Point::new(x, y))
        })
        .next()
        .unwrap();

    // std::iter::successors(Some([start].into_iter().clone()), |p| {
    //     Some(
    //         p.map(|p| {
    //             p.neighbors()
    //                 .into_iter()
    //                 .filter(|p| matches!(grid[*p], Tile::Plot))
    //         })
    //         .flatten(),
    //     )
    // });

    // let mut reachable: Grid<bool> = grid.empty_sized();
    // reachable[start] = true;
    //
    // for _ in 0..steps {
    //     let mut new_reachable = reachable.empty_sized();
    //     let points = reachable
    //         .flat_iter()
    //         .filter(|(b, _)| **b)
    //         .map(|(_, p)| reachable.neighbors_of(p))
    //         .flatten()
    //         .filter(|p| grid.get_wrapping(p) == Tile::Plot);
    //
    //     for p in points {
    //         new_reachable[p] = true;
    //     }
    //
    //     reachable = new_reachable;
    // }
    //
    // reachable.print(|b| if *b { 'O' } else { '.' });
    //
    // reachable.count(|b, _| *b)

    todo!()
}

fn solve(input: &str, steps: usize) -> usize {
    let grid: Grid<Tile> = Grid::from_chars(input);
    let start = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'S')
                .map(move |(x, _)| Point::new(x, y))
        })
        .next()
        .expect("starting position");

    let mut reachable: Grid<bool> = grid.empty_sized();
    reachable[start] = true;

    for _ in 0..steps {
        let mut new_reachable = reachable.empty_sized();
        let points = reachable
            .flat_iter()
            .filter(|(b, _)| **b)
            .map(|(_, p)| reachable.neighbors_of(p))
            .flatten()
            .filter(|p| matches!(grid[*p], Tile::Plot));

        for p in points {
            new_reachable[p] = true;
        }

        reachable = new_reachable;
    }

    reachable.print(|b| if *b { 'O' } else { '.' });

    reachable.count(|b, _| *b)
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(1, 2)]
    #[case(2, 4)]
    #[case(3, 6)]
    #[case(6, 16)]
    fn test_solve(#[case] steps: usize, #[case] expected: usize) {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve(&input, steps), expected);
    }

    #[rstest]
    #[case(1, 2)]
    #[case(2, 4)]
    #[case(3, 6)]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_solve_wrapping(#[case] steps: usize, #[case] expected: usize) {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(solve_wrapping(&input, steps), expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
