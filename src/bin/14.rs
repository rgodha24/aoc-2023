advent_of_code::solution!(14);
use std::fmt::Display;

use advent_of_code::helpers::*;

pub fn part_one(input: &str) -> Option<usize> {
    let mut m = Mirror::new(input);

    m.roll_north();

    Some(m.load())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m = Mirror::new(input);
    let mut cycles = Cycleable::new();

    loop {
        cycles.add(m.0.clone());

        if cycles.cycle_found() {
            break;
        }

        m.cycle();
    }
    let ans = cycles.at_cycled(1000000000).unwrap();

    let m = Mirror(ans.clone());

    Some(m.load())
}

#[derive(Debug, Clone, PartialEq)]
struct Mirror(Grid<Tile>);

impl Mirror {
    fn new(input: &str) -> Self {
        let g = Grid::from_chars(input);
        Self(g)
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn roll_north(&mut self) {
        for y in 1..self.0.height() {
            for x in 0..self.0.width() {
                let t = self.0[Point::new(x, y)];
                if matches!(t, Tile::Empty | Tile::CubeRock) {
                    continue;
                }
                for j in (0..y).rev() {
                    let p = Point::new(x, j + 1);
                    let above = Point::new(x, j);
                    let a = self.0[above];
                    let c = self.0[p];

                    if let (Tile::Empty, Tile::RoundRock) = (a, c) {
                        self.0[above] = Tile::RoundRock;
                        self.0[p] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for y in (0..self.0.height() - 1).rev() {
            for x in 0..self.0.width() {
                let t = self.0[Point::new(x, y)];
                if matches!(t, Tile::Empty | Tile::CubeRock) {
                    continue;
                }
                for j in (y + 1)..self.0.height() {
                    let p = Point::new(x, j - 1);
                    let above = Point::new(x, j);
                    let a = self.0[above];
                    let c = self.0[p];

                    if let (Tile::Empty, Tile::RoundRock) = (a, c) {
                        self.0[above] = Tile::RoundRock;
                        self.0[p] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for x in 1..self.0.width() {
            for y in 0..self.0.height() {
                let t = self.0[Point::new(x, y)];
                if matches!(t, Tile::Empty | Tile::CubeRock) {
                    continue;
                }
                for j in (0..x).rev() {
                    let p = Point::new(j + 1, y);
                    let above = Point::new(j, y);
                    let a = self.0[above];
                    let c = self.0[p];

                    if let (Tile::Empty, Tile::RoundRock) = (a, c) {
                        self.0[above] = Tile::RoundRock;
                        self.0[p] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for x in (0..self.0.width() - 1).rev() {
            for y in 0..self.0.height() {
                let t = self.0[Point::new(x, y)];
                if matches!(t, Tile::Empty | Tile::CubeRock) {
                    continue;
                }
                for j in x + 1..self.0.width() {
                    let p = Point::new(j - 1, y);
                    let above = Point::new(j, y);
                    let a = self.0[above];
                    let c = self.0[p];

                    if let (Tile::Empty, Tile::RoundRock) = (a, c) {
                        self.0[above] = Tile::RoundRock;
                        self.0[p] = Tile::Empty;
                    }
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.0
            .flat_iter()
            .filter(|(t, _)| matches!(t, Tile::RoundRock))
            .map(|(_, p)| self.0.height() - p.y_u())
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    RoundRock,
    CubeRock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_cycles() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let c1 = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;

        let c2 = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#;

        let c3 = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;

        let mut mirror = Mirror::new(input);
        let c1 = Mirror::new(c1);
        let c2 = Mirror::new(c2);
        let c3 = Mirror::new(c3);

        mirror.cycle();
        assert_eq!(mirror, c1);

        mirror.cycle();
        assert_eq!(mirror, c2);

        mirror.cycle();
        assert_eq!(mirror, c3);
    }
}
