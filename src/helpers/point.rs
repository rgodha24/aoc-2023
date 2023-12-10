#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Add, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// creates a new point from the given coordinates
    pub fn new_checked<T: TryInto<isize>>(x: T, y: T) -> Result<Self, ()> {
        Ok(Self {
            x: x.try_into().map_err(|_| ())?,
            y: y.try_into().map_err(|_| ())?,
        })
    }

    pub fn new<T: TryInto<isize>>(x: T, y: T) -> Self {
        Self::new_checked(x, y).unwrap()
    }

    /// returns the manhattan distance between two points
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    /// returns `x` in the type `K`
    pub fn x<K: TryFrom<isize>>(&self) -> K {
        match self.x.try_into() {
            Ok(k) => k,
            Err(_) => panic!("x can't be converted to K"),
        }
    }
    /// returns `y` in the type `K`
    pub fn y<K: TryFrom<isize>>(&self) -> K {
        match self.y.try_into() {
            Ok(k) => k,
            Err(_) => panic!("y can't be converted to K"),
        }
    }

    pub fn x_u(&self) -> usize {
        self.x::<usize>()
    }
    pub fn y_u(&self) -> usize {
        self.y::<usize>()
    }

    pub fn neighbors(&self) -> [Self; 4] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }
    pub fn neighbors_diag(&self) -> [Self; 8] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(f, "({}, {})", x, y)
    }
}

impl<T> From<(T, T)> for Point
where
    T: TryInto<isize>,
{
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
