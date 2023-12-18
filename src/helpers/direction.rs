#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

use std::ops::{Add, AddAssign, Sub, SubAssign};

use Direction::*;

use super::Point;

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
        }
    }

    /// returns every direction except for self
    pub fn except_self(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down, Left],
            Left => vec![Up, Down, Right],
            Up => vec![Left, Right, Down],
            Down => vec![Left, Right, Up],
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Right, Left, Up, Down]
    }

    pub fn except_self_and_opposite(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down],
            Left => vec![Up, Down],
            Up => vec![Left, Right],
            Down => vec![Left, Right],
        }
    }

    pub fn except_opposite(&self) -> Vec<Self> {
        match self {
            Right => vec![Up, Down, Right],
            Left => vec![Up, Down, Left],
            Up => vec![Left, Right, Down],
            Down => vec![Left, Right, Up],
        }
    }

    pub fn as_point(&self) -> Point {
        match self {
            Right => Point::new(1, 0),
            Left => Point::new(-1, 0),
            Up => Point::new(0, -1),
            Down => Point::new(0, 1),
        }
    }
}

impl Add<&Direction> for Point {
    type Output = Point;

    fn add(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

impl Sub<&Direction> for Point {
    type Output = Point;

    fn sub(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x + 1, self.y),
            Direction::Right => Point::new(self.x - 1, self.y),
        }
    }
}

impl AddAssign<&Direction> for Point {
    fn add_assign(&mut self, rhs: &Direction) {
        match rhs {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}
impl SubAssign<&Direction> for Point {
    fn sub_assign(&mut self, rhs: &Direction) {
        match rhs {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' | 'r' | '>' => Right,
            'L' | 'l' | '<' => Left,
            'U' | 'u' | '^' => Up,
            'D' | 'd' | 'v' => Down,
            c => panic!("invalid direction char: {}", c),
        }
    }
}
