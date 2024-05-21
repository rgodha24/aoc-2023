advent_of_code::solution!(17);
use std::{
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

use advent_of_code::helpers::*;
use itertools::Itertools;

use Direction::*;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_lines(input, |l| {
        l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    });
    let end = Point::new(grid.width() - 1, grid.height() - 1);
    println!("end: {end}");

    let mut heap: BinaryHeap<Location<P1>> = [
        Location::new(Point::new(1, 0), Right),
        Location::new(Point::new(0, 1), Down),
    ]
    .into();

    let mut visited: HashSet<(Point, Direction, usize)> = HashSet::new();
    let mut best = u32::MAX;

    while let Some(location) = heap.pop() {
        let our_cost = location.cost + grid[location.pos];
        if our_cost >= best {
            continue;
        }

        if location.pos == end {
            if our_cost < best {
                println!("new best: {}", our_cost);
                best = our_cost;
            }
            continue;
        }

        if !visited.insert((location.pos, location.direction, location.amount)) {
            continue;
        }

        for next_location in location
            .next_locs(grid[location.pos])
            .filter(|l| grid.contains_point(l.pos))
        {
            heap.push(next_location);
        }
    }

    Some(best)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_lines(input, |l| {
        l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    });
    let end = Point::new(grid.width() - 1, grid.height() - 1);
    println!("end: {end}");

    let mut heap: BinaryHeap<Location<P2>> = [
        Location::new(Point::new(1, 0), Right),
        Location::new(Point::new(0, 1), Down),
    ]
    .into();

    // let mut visited: HashSet<(Point, Direction, usize)> = HashSet::new();
    let mut best = u32::MAX;

    while let Some(location) = heap.pop() {
        let our_cost = location.cost + grid[location.pos];
        if our_cost >= best {
            continue;
        }

        if location.pos == end {
            if our_cost < best && location.amount >= 4 {
                println!("new best: {} loc: {:?}", our_cost, location);
                best = our_cost;
            }
            continue;
        }

        // if !visited.insert((location.pos, location.direction, location.amount)) {
        //     continue;
        // }

        for next_location in location
            .next_locs(grid[location.pos])
            .filter(|l| grid.contains_point(l.pos))
        {
            heap.push(next_location);
        }
    }

    Some(best)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location<P> {
    pos: Point,
    direction: Direction,
    amount: usize,
    cost: u32,
    phantom: std::marker::PhantomData<P>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct P1;
#[derive(Debug, Clone, PartialEq, Eq)]
struct P2;

impl Location<P1> {
    fn next_locs(&self, extra_cost: u32) -> impl Iterator<Item = Location<P1>> + '_ {
        let opposite_dir = self.direction.opposite();

        [Right, Left, Up, Down]
            .into_iter()
            .filter(move |d| *d != opposite_dir)
            .filter_map(move |d| self.clone() + d)
            .map(move |mut l| {
                l.cost += extra_cost;
                l
            })
    }
}

impl Location<P2> {
    fn next_locs(&self, extra_cost: u32) -> impl Iterator<Item = Location<P2>> + '_ {
        (if self.amount < 4 {
            vec![self.direction]
        } else if self.amount == 10 {
            self.direction.except_self_and_opposite()
        } else {
            self.direction.except_opposite()
        })
        .into_iter()
        .filter_map(move |d| self.clone() + d)
        // .inspect(|l| println!("next: {:?} from {}", l, self.pos))
        .map(move |mut l| {
            l.cost += extra_cost;
            l
        })
    }
}

impl<P> Location<P> {
    fn new(pos: Point, start_dir: Direction) -> Self {
        Self {
            pos,
            direction: start_dir,
            amount: 1,
            cost: 0,
            phantom: std::marker::PhantomData,
        }
    }

    fn new_amt(pos: Point, start_dir: Direction, amount: usize) -> Self {
        Self {
            pos,
            direction: start_dir,
            amount,
            cost: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

impl Add<Direction> for Location<P1> {
    type Output = Option<Location<P1>>;

    fn add(mut self, rhs: Direction) -> Self::Output {
        let new_pos = self.pos + &rhs;
        if new_pos.x < 0 || new_pos.y < 0 {
            return None;
        }

        self.pos = new_pos;

        if self.direction == rhs {
            if self.amount == 3 {
                return None;
            } else {
                self.amount += 1;
            }
        } else {
            self.direction = rhs;
            self.amount = 1;
        }

        Some(self)
    }
}

impl Add<Direction> for Location<P2> {
    type Output = Option<Location<P2>>;

    fn add(mut self, rhs: Direction) -> Self::Output {
        let new_pos = self.pos + &rhs;
        if new_pos.x < 0 || new_pos.y < 0 {
            return None;
        }

        self.pos = new_pos;

        if self.direction == rhs {
            self.amount += 1;
        } else {
            self.direction = rhs;
            self.amount = 1;
        }

        Some(self)
    }
}

impl<P: Eq> Ord for Location<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<P: Eq> PartialOrd for Location<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[rstest]
    #[case(r#"111111111111
999999999991
999999999991
999999999991
999999999991"#.to_string(), 71)]
    #[case(advent_of_code::template::read_file("examples", DAY), 94)]
    fn test_part_two(#[case] input: String, #[case] expected: u32) {
        let result = part_two(&input);
        assert_eq!(result, Some(expected));
    }

    // #[rstest]
    // #[case(Location::new_amt(Point::new(9,0), Right, 9), vec![Right, Down])]
    // fn test_p2_neighbors(#[case] l: Location<P2>, #[case] dirs: Vec<Direction>) {
    //     let mut neighbors = l.neighbors(0);
    //     let mut expected = dirs
    //         .into_iter()
    //         .map(|d| {
    //             let mut l = l.clone();
    //             l.cost += 1;
    //             l + d
    //         })
    //         .collect::<Vec<_>>();
    //     neighbors.sort();
    //     expected.sort();
    //     assert_eq!(neighbors, expected);
    // }
}
