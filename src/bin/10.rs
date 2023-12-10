advent_of_code::solution!(10);
use advent_of_code::helpers::{Grid, Point};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<isize> {
    let tiles = parse(input);
    let distances = get_distances(&tiles);

    distances
        .flat_iter()
        .map(|(d, _)| d)
        .max()
        .cloned()
        .flatten()
}

pub fn part_two(input: &str) -> Option<usize> {
    let tiles = parse(input);
    let distances = get_distances(&tiles);

    Some(
        distances
            .flat_iter()
            .map(|(d, p)| {
                if d.is_some() {
                    return 0;
                }

                // point-in-polygon raycasting algorithm
                // see https://arc.net/l/quote/scsdxdiz
                let intersections_left = (0..p.x())
                    .map(|x| Point::new(x, p.y()))
                    // we only care about pipes that are in the main loop
                    .filter(|&p| distances[p].is_some())
                    // if we have F-----J or L------7 (case 1), it's only one intersection
                    // something F---7 or L--J (case 2), it's not an intersection at all
                    // therefore, we only match on TR (L) and RL (J) bends because:
                    // it is 1 intersection for case 1, and 2 intersections for case 2
                    // (which doesn't matter because we only care if it's odd or even)
                    .filter(|&p| matches!(tiles[p], Tile::Vertical | Tile::TRBend | Tile::TLBend))
                    .count();

                // same thing as above but going to the right
                let intersections_right = (p.x()..tiles.width())
                    .map(|x| Point::new(x, p.y()))
                    .filter(|&p| distances[p].is_some())
                    .filter(|&p| matches!(tiles[p], Tile::Vertical | Tile::TRBend | Tile::TLBend))
                    .count();

                if (intersections_left % 2 == 1) && (intersections_right % 2 == 1) {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::from_chars(input)
}

fn get_distances(tiles: &Grid<Tile>) -> Grid<Option<isize>> {
    let start = tiles
        .flat_iter()
        .find_map(|(&t, p)| if t == Tile::Start { Some(p) } else { None })
        .unwrap();

    let mut distances = tiles.empty_sized();
    let mut queue = Vec::new();

    distances[start] = Some(0);

    // start the queue with the 2 neighbors of the start
    for (p, t) in tiles.neighbors_of(start).into_iter().map(|p| (p, tiles[p])) {
        if t == Tile::Ground {
            continue;
        }

        let delta = p - start;
        if tiles[p].possible_neighbors().contains(&(-delta)) {
            distances[p] = Some(1);
            queue.push((p, 1));
        }
    }

    assert_eq!(queue.len(), 2);

    while let Some((point, distance)) = queue.pop() {
        let tile = tiles[point];
        // theoretically not necessary
        if tile == Tile::Ground {
            continue;
        }

        let neighbors = tile
            .possible_neighbors()
            .map(|delta| point + delta)
            .filter(|&p| tiles.contains_point(p));

        for n in neighbors {
            let neighbor = tiles[n];
            if neighbor == Tile::Ground {
                continue;
            }

            let current_distance = distances[n];
            if current_distance.unwrap_or(isize::MAX) > distance + 1 {
                distances[n] = Some(distance + 1);
                queue.push((n, distance + 1));
            }
        }
    }

    distances
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    Vertical,
    Horizontal,
    /// J
    TLBend,
    /// L
    TRBend,
    /// 7
    BLBend,
    /// F
    BRBend,
    #[default]
    Ground,
    Start,
}

impl Tile {
    fn possible_neighbors(&self) -> impl Iterator<Item = Point> {
        (match self {
            Tile::Vertical => vec![(0, 1), (0, -1)],
            Tile::Horizontal => vec![(1, 0), (-1, 0)],
            Tile::TLBend => vec![(0, -1), (-1, 0)],
            Tile::TRBend => vec![(0, -1), (1, 0)],
            Tile::BLBend => vec![(0, 1), (-1, 0)],
            Tile::BRBend => vec![(0, 1), (1, 0)],
            Tile::Ground => panic!("neighbors of ground"),
            Tile::Start => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        })
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::TRBend,
            'J' => Tile::TLBend,
            '7' => Tile::BLBend,
            'F' => Tile::BRBend,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unknown tile: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let result = part_one(input);
        assert_eq!(result, Some(4));

        let input = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;
        let result = part_one(input);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        assert_eq!(part_two(input), Some(4));

        let input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;
        assert_eq!(part_two(input), Some(4));

        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        assert_eq!(part_two(input), Some(8));

        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(part_two(input), Some(10));
    }
}
