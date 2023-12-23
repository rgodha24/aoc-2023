use std::ops::{Index, IndexMut};

use super::{Grid, Point3};

pub struct Grid3<T> {
    grids: Vec<Grid<T>>,
}

impl<T> Grid3<T> {
    pub fn empty(x: usize, y: usize, z: usize) -> Self
    where
        T: Clone + Default,
    {
        Self {
            grids: vec![Grid::empty(x, y); z],
        }
    }

    pub fn count(&self, f: impl Fn(&T, Point3) -> bool) -> u64 {
        let mut count = 0;
        for (z, grid) in self.grids.iter().enumerate() {
            for (item, p) in grid.flat_iter() {
                if f(item, Point3::new_p(p, z as isize)) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl<T> Index<Point3> for Grid3<T> {
    type Output = T;

    fn index(&self, index: Point3) -> &Self::Output {
        &self.grids[index.z as usize][index.p]
    }
}

impl<T> IndexMut<Point3> for Grid3<T> {
    fn index_mut(&mut self, index: Point3) -> &mut Self::Output {
        &mut self.grids[index.z as usize][index.p]
    }
}
