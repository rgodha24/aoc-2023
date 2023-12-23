use std::{ops::Add, str::FromStr};

use itertools::Itertools;

use super::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub z: isize,
    pub p: Point,
}

impl Point3 {
    pub fn new<T: TryInto<isize>>(x: T, y: T, z: T) -> Self {
        Self::new_checked(x, y, z).unwrap()
    }
    pub fn new_checked<T: TryInto<isize>>(x: T, y: T, z: T) -> Result<Self, ()> {
        Ok(Self {
            z: z.try_into().map_err(|_| ())?,
            p: Point::new_checked(x, y)?,
        })
    }

    pub fn new_p(p: Point, z: isize) -> Self {
        Self { z, p }
    }
}

impl FromStr for Point3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect_tuple()
            .ok_or(())?;

        Self::new_checked(x, y, z)
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Self::new(self.p.x + rhs.p.x, self.p.y + rhs.p.y, self.z + rhs.z)
    }
}
