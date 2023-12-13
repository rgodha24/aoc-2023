#![allow(dead_code)]

use std::{
    default::Default,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use derive_more::{Deref, DerefMut};

use super::Point;

#[derive(Deref, DerefMut, Clone, PartialEq, Hash)]
/// 2D grid of data stored in Row-Major order by default
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
    /// returns width in K integer type
    pub fn width_n<K: TryFrom<usize>>(&self) -> K {
        match self.data[0].len().try_into() {
            Ok(k) => k,
            Err(_) => panic!("width can't be converted to K"),
        }
    }
    /// returns height in K integer type
    pub fn height_n<K: TryFrom<usize>>(&self) -> K {
        match self.data.len().try_into() {
            Ok(k) => k,
            Err(_) => panic!("height can't be converted to K"),
        }
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = (&T, Point)> {
        self.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, col)| (col, Point::new(x, y)))
        })
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        itertools::iproduct!(0..self.width(), 0..self.height()).map(|(x, y)| Point::new(x, y))
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }
    pub fn height(&self) -> usize {
        self.data.len()
    }
    pub fn from_lines(input: &str, line: impl Fn(&str) -> Vec<T>) -> Self {
        let data = input.lines().map(line).collect();
        Self { data }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width_n() && point.y < self.height_n()
    }

    /// create a Grid from a string of characters
    /// the bottom left corner is (0, 0)
    pub fn from_chars(input: &str) -> Self
    where
        T: From<char>,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();
        Self { data }
    }

    /// iterates over the grid, allowing for mutations
    pub fn for_each_mut(&mut self, f: impl Fn(&mut T, Point)) {
        for (y, row) in self.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                f(col, Point::new(x, y));
            }
        }
    }

    /// iterates over grid, immutably
    pub fn for_each(&self, f: impl Fn(&T, Point)) {
        for (y, row) in self.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                f(col, Point::new(x, y));
            }
        }
    }

    pub fn neighbors_of(&self, p: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let Point { x, y } = p;
        if x > 0 {
            neighbors.push(Point::new(x - 1, y));
        }
        if y > 0 {
            neighbors.push(Point::new(x, y - 1));
        }

        let width: isize = self.width_n();
        let height: isize = self.height_n();

        if x < width - 1 {
            neighbors.push(Point::new(x + 1, y));
        }
        if y < height - 1 {
            neighbors.push(Point::new(x, y + 1));
        }
        neighbors
    }

    pub fn neighbors_of_diagonal(&self, p: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let width: isize = self.width_n();
        let height: isize = self.height_n();
        let Point { x, y } = p;
        if x > 0 {
            neighbors.push(Point::new(x - 1, y));
        }
        if y > 0 {
            neighbors.push(Point::new(x, y - 1));
        }
        if x < width - 1 {
            neighbors.push(Point::new(x + 1, y));
        }
        if y < height - 1 {
            neighbors.push(Point::new(x, y + 1));
        }
        if x > 0 && y > 0 {
            neighbors.push(Point::new(x - 1, y - 1));
        }
        if x < width - 1 && y > 0 {
            neighbors.push(Point::new(x + 1, y - 1));
        }
        if x > 0 && y < height - 1 {
            neighbors.push(Point::new(x - 1, y + 1));
        }
        if x < width - 1 && y < height - 1 {
            neighbors.push(Point::new(x + 1, y + 1));
        }

        neighbors
    }

    pub fn neighbors_of_filtered(&self, p: Point, f: impl Fn(&T, &Point) -> bool) -> Vec<Point> {
        self.neighbors_of(p)
            .into_iter()
            .filter(|p| f(&self[*p], p))
            .collect()
    }

    pub fn neighbors_of_diagonal_filtered(
        &self,
        p: Point,
        f: impl Fn(&T, &Point) -> bool,
    ) -> Vec<Point> {
        self.neighbors_of_diagonal(p)
            .into_iter()
            .filter(|p| f(&self[*p], p))
            .collect()
    }

    /// combine 2 grids into a grid of tuples
    pub fn combine2<T1, T2>(g1: Grid<T1>, g2: Grid<T2>) -> Grid<(T1, T2)> {
        let mut data = Vec::new();
        for (row1, row2) in g1.data.into_iter().zip(g2.data.into_iter()) {
            let mut row = Vec::new();
            for (col1, col2) in row1.into_iter().zip(row2.into_iter()) {
                row.push((col1, col2));
            }
            data.push(row);
        }
        Grid { data }
    }

    /// combine 3 grids into a grid of tuples
    pub fn combine3<T1, T2, T3>(g1: Grid<T1>, g2: Grid<T2>, g3: Grid<T3>) -> Grid<(T1, T2, T3)> {
        let mut data = Vec::new();
        for ((row1, row2), row3) in g1
            .data
            .into_iter()
            .zip(g2.data.into_iter())
            .zip(g3.data.into_iter())
        {
            let mut row = Vec::new();
            for ((col1, col2), col3) in row1.into_iter().zip(row2.into_iter()).zip(row3.into_iter())
            {
                row.push((col1, col2, col3));
            }
            data.push(row);
        }
        Grid { data }
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.data.iter()
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
        let width = self.width();
        (0..width).map(move |i| self.data.iter().map(move |row| &row[i]).collect())
    }

    pub fn row(&self, y: usize) -> &Vec<T> {
        &self.data[y]
    }

    pub fn col(&self, x: usize) -> Vec<&T> {
        self.data.iter().map(|row| &row[x]).collect()
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Clone + Default> Grid<T> {
    /// swaps from Row-Major to Column-Major order
    /// and vice-versa
    pub fn swap_order(&mut self) {
        let rows = self.len();
        let cols = self.data[0].len();

        // Create a new grid with reversed order
        let mut new_data = self.empty_sized();

        for i in 0..rows {
            for j in 0..cols {
                std::mem::swap(&mut new_data.data[j][i], &mut self.data[i][j]);
            }
        }

        // Update the grid with the new order
        self.data = new_data.data;
    }

    pub fn insert_empty_row(&mut self, index: usize) {
        self.data
            .insert(index, vec![Default::default(); self.width()]);
    }

    pub fn insert_empty_col(&mut self, index: usize) {
        for row in self.data.iter_mut() {
            row.insert(index, Default::default());
        }
    }
}

impl<T> Grid<T> {
    /// creates an empty grid with the same dimensions as self
    pub fn empty_sized<K: Clone + Default>(&self) -> Grid<K> {
        let data = vec![vec![Default::default(); self.width()]; self.height()];

        Grid { data }
    }

    /// maps over self to create a new Grid
    pub fn map<K: Clone + Default>(&self, f: impl Fn(&T, Point) -> K) -> Grid<K> {
        let mut new_data = self.empty_sized();
        for (y, row) in self.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let p = Point::new(x, y);
                new_data[p] = f(col, p);
            }
        }
        new_data
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y_u()][index.x_u()]
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
