advent_of_code::solution!(8);
use std::collections::BTreeMap;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (dirs, grid) = parse(input);

    Some(time_to("AAA", &dirs, &grid, |s| s == "ZZZ"))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (dirs, grid) = parse(input);

    Some(math::lcm(
        grid.keys()
            .filter(|&k| k.ends_with('A'))
            .map(|&s| time_to(s, &dirs, &grid, |s| s.ends_with('Z'))),
    ))
}

enum Dir {
    R,
    L,
}

fn time_to(
    start: &str,
    dirs: &[Dir],
    grid: &BTreeMap<&str, (&str, &str)>,
    is_done: impl Fn(&str) -> bool,
) -> usize {
    let mut curr = start;

    for i in 0usize.. {
        let d = &dirs[i.rem_euclid(dirs.len())];

        let (left, right) = grid.get(curr).unwrap();

        let next = match d {
            Dir::R => right,
            Dir::L => left,
        };

        if is_done(next) {
            return i + 1;
        }

        curr = next;
    }

    panic!("no path to Z");
}

fn parse(input: &str) -> (Vec<Dir>, BTreeMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let dirs = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => unreachable!(),
        })
        .collect_vec();

    // whitespace
    lines.next();

    let grid: BTreeMap<&str, (&str, &str)> = lines
        .map(|l| {
            let (start, rest) = l.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            let left = left.trim_start_matches('(');
            let right = right.trim_end_matches(')');

            (start, (left, right))
        })
        .collect();

    (dirs, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
