advent_of_code::solution!(18);

use std::str::FromStr;

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<isize> {
    Some(solve::<P1>(input))
}

pub fn part_two(input: &str) -> Option<isize> {
    Some(solve::<P2>(input))
}

fn solve<T: Dig>(input: &str) -> isize
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut digs = input.lines().map(T::from_str).map(Result::unwrap);

    let mut boundaries = 0;
    let points = std::iter::successors(Some(Point::new(0, 0)), |p| {
        let d = digs.next()?;
        boundaries += d.amount();
        let dir = d.direction().as_point();
        Some(*p + dir * d.amount())
    })
    .collect_vec();

    let area = math::shoelace(&points);
    let inside = math::picks(area, boundaries);

    boundaries + inside
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct P1 {
    direction: Direction,
    amount: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct P2 {
    direction: Direction,
    amount: isize,
}

trait Dig: FromStr {
    fn direction(&self) -> Direction;
    fn amount(&self) -> isize;
}

impl Dig for P1 {
    fn direction(&self) -> Direction {
        self.direction
    }
    fn amount(&self) -> isize {
        self.amount
    }
}

impl Dig for P2 {
    fn direction(&self) -> Direction {
        self.direction
    }
    fn amount(&self) -> isize {
        self.amount
    }
}

impl FromStr for P1 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount, _) = s.splitn(3, ' ').collect_tuple().ok_or(())?;
        let direction = direction.chars().next().ok_or(())?;

        Ok(Self {
            direction: direction.into(),
            amount: amount.parse().unwrap(),
        })
    }
}

impl FromStr for P2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        let (_, color) = s.split_once('#').ok_or(())?;
        let amount = &color[0..5];
        let amount = isize::from_str_radix(amount, 16).map_err(|_| ())?;

        let direction = color.chars().nth(5).ok_or(())?;
        let direction = match direction {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => return Err(()),
        };

        Ok(Self { direction, amount })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
