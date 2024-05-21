advent_of_code::solution!(24);
use std::str::FromStr;

use advent_of_code::helpers::*;
use itertools::Itertools;
use rug::Rational;

pub fn part_one(input: &str) -> Option<u32> {
    let min = Rational::from(200000000000000u64);
    let max = Rational::from(400000000000000u64);
    let count = input
        .lines()
        .map(|l| l.parse::<Hailstone>().unwrap())
        .tuple_combinations()
        .filter_map(|(a, b)| intersection(&a, &b))
        .filter(|(x, y)| x >= &min && x <= &max && y >= &min && y <= &max)
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn intersection(a: &Hailstone, b: &Hailstone) -> Option<(Rational, Rational)> {
    let (a1, b1, c1) = (a.a(), a.b(), a.c());
    let (a2, b2, c2) = (b.a(), b.b(), b.c());

    // parallel
    if a1 * b2 == a2 * b1 {
        return None;
    }

    let x = Rational::from((b2 * c1 - b1 * c2, a1 * b2 - a2 * b1));
    let y = Rational::from((a1 * c2 - a2 * c1, a1 * b2 - a2 * b1));

    // in the past
    if (x.clone() - a.s.p.x) * a.v.p.x < 0
        || (x.clone() - b.s.p.x) * b.v.p.x < 0
        || (y.clone() - a.s.p.y) * a.v.p.y < 0
        || (y.clone() - b.s.p.y) * b.v.p.y < 0
    {
        return None;
    }

    Some((x, y))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hailstone {
    s: Point3,
    v: Point3,
}

impl Hailstone {
    fn a(&self) -> isize {
        self.v.p.y
    }
    fn b(&self) -> isize {
        -self.v.p.x
    }
    fn c(&self) -> isize {
        self.s.p.x * self.v.p.y - self.s.p.y * self.v.p.x
    }
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, v) = s.split_once(" @ ").ok_or(())?;
        let s = s.parse()?;
        let v = v.parse()?;

        Ok(Self { s, v })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("19, 13, 30 @ -2, 1, -2", "18, 19, 22 @ -1, -1, -2", Some(("43/3", "46/3")))]
    #[case("19, 13, 30 @ -2, 1, -2", "20, 25, 34 @ -2, -2, -4", Some(("35/3", "50/3")))]
    #[case("19, 13, 30 @ -2, 1, -2", "12, 31, 28 @ -1, -2, -1", Some(("31/5", "97/5")))]
    #[case("19, 13, 30 @ -2, 1, -2", "20, 19, 15 @ 1, -5, -3", None)]
    #[case("18, 19, 22 @ -1, -1, -2", "20, 25, 34 @ -2, -2, -4", None)]
    #[case("18, 19, 22 @ -1, -1, -2", "12, 31, 28 @ -1, -2, -1", Some(("-6", "-5")))]
    #[case("18, 19, 22 @ -1, -1, -2", "20, 19, 15 @ 1, -5, -3", None)]
    #[case("20, 25, 34 @ -2, -2, -4", "12, 31, 28 @ -1, -2, -1", Some(("-2", "3")))]
    #[case("20, 25, 34 @ -2, -2, -4", "20, 19, 15 @ 1, -5, -3", None)]
    #[case("12, 31, 28 @ -1, -2, -1", "20, 19, 15 @ 1, -5, -3", None)]
    fn test_collision(
        #[case] a: Hailstone,
        #[case] b: Hailstone,
        #[case] expected: Option<(&str, &str)>,
    ) {
        let expected = expected.map(|(a, b)| {
            (
                Rational::from_str(a).unwrap(),
                Rational::from_str(b).unwrap(),
            )
        });
        let i = intersection(&a, &b);
        assert_eq!(i, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
