advent_of_code::solution!(22);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (bricks, (x, y, z)) = parse(input);
    let mut grid: Grid3<bool> = Grid3::empty(x, y, z);

    for p in bricks.iter().map(|b| b.inside()).flatten() {
        grid[p] = true;
    }

    let count = bricks
        .iter()
        .inspect(|b| println!("{:?}", b))
        .filter(|b| !b.under().all(|p| !grid[p]))
        .inspect(|b| println!("{:?}", b))
        .count();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> (Vec<Brick>, (usize, usize, usize)) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let bricks = input
        .lines()
        .map(|l| {
            let (p1, p2) = l.split_once('~').unwrap();
            let p1 = p1.parse::<Point3>().unwrap();
            let p2 = p2.parse::<Point3>().unwrap();

            max_x = max_x.max(p1.p.x).max(p2.p.x);
            max_y = max_y.max(p1.p.y).max(p2.p.y);
            max_z = max_z.max(p1.z).max(p2.z);

            (p1, p2).into()
        })
        .collect();

    (
        bricks,
        (max_x as usize + 1, max_y as usize + 1, max_z as usize + 1),
    )
}

/// a brick can either extend in the x, y, or z direction
#[derive(Debug, Clone, PartialEq, Eq)]
enum Brick {
    X {
        x: (isize, isize),
        y: isize,
        z: isize,
    },
    Y {
        x: isize,
        y: (isize, isize),
        z: isize,
    },
    Z {
        x: isize,
        y: isize,
        z: (isize, isize),
    },
}

impl Brick {
    fn under(&self) -> Box<dyn Iterator<Item = Point3> + '_> {
        match self {
            Brick::X { x, y, z } => Box::new((x.0..=x.1).map(move |x| Point3::new(x, *y, *z - 1))),
            Brick::Y { x, y, z } => Box::new((y.0..=y.1).map(move |y| Point3::new(*x, y, *z - 1))),
            Brick::Z { x, y, z } => Box::new((z.0..=z.1).map(move |z| Point3::new(*x, *y, z - 1))),
        }
    }

    fn inside(&self) -> Box<dyn Iterator<Item = Point3> + '_> {
        match self {
            Brick::X { x, y, z } => Box::new((x.0..=x.1).map(move |x| Point3::new(x, *y, *z))),
            Brick::Y { x, y, z } => Box::new((y.0..=y.1).map(move |y| Point3::new(*x, y, *z))),
            Brick::Z { x, y, z } => Box::new((z.0..=z.1).map(move |z| Point3::new(*x, *y, z))),
        }
    }
}

impl From<(Point3, Point3)> for Brick {
    fn from((p1, p2): (Point3, Point3)) -> Self {
        if p1.z != p2.z {
            Brick::Z {
                x: p1.p.x,
                y: p1.p.y,
                z: (p1.z, p2.z),
            }
        } else if p1.p.x != p2.p.x {
            Brick::X {
                x: (p1.p.x, p2.p.x),
                y: p1.p.y,
                z: p1.z,
            }
        } else if p1.p.y != p2.p.y {
            Brick::Y {
                x: p1.p.x,
                y: (p1.p.y, p2.p.y),
                z: p1.z,
            }
        } else {
            panic!("Invalid brick")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
