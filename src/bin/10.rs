advent_of_code::solution!(10);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<isize> {
    let tiles = parse(input);
    let distances = get_distances(&tiles);

    distances.into_iter().flatten().max().flatten()
}

pub fn part_two(input: &str) -> Option<usize> {
    let tiles = parse(input);
    let distances = get_distances(&tiles);

    let mut sum = 0;

    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            let d = distances[y][x];
            // something thats part of the loop can not be inside the loop
            if d.is_some() {
                continue;
            }

            // point-in-polygon raycasting algorithm
            // see https://arc.net/l/quote/scsdxdiz
            let intersections_left = (0..x)
                // we only care about pipes that are in the main loop
                .filter(|x| distances[y][*x].is_some())
                // if we have F-----J or L------7 (case 1), it's only one intersection
                // something F---7 or L--J (case 2), it's not an intersection at all
                // therefore, we only match on TR (L) and RL (J) bends because it is 1 intersection
                // for case 1, and 2 (which doesn't matter because we only care if it's odd or even)
                // for case 2.
                .filter(|x| matches!(tiles[y][*x], Tile::Vertical | Tile::TRBend | Tile::TLBend))
                .count();

            // same thing as above but going to the right
            let intersections_right = (x..tiles[y].len())
                .filter(|x| distances[y][*x].is_some())
                .filter(|x| matches!(tiles[y][*x], Tile::Vertical | Tile::TRBend | Tile::TLBend))
                .count();

            if (intersections_left % 2 == 1) && (intersections_right % 2 == 1) {
                sum += 1;
            }
        }
    }

    Some(sum)
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| l.chars().map(Tile::from).collect_vec())
        .collect_vec()
}

fn get_distances(tiles: &[Vec<Tile>]) -> Vec<Vec<Option<isize>>> {
    let start = tiles
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut distances = vec![vec![None; tiles[0].len()]; tiles.len()];
    // (x, y), distance
    let mut queue = vec![(start, 0)];

    distances[start.1][start.0] = Some(0);

    while let Some(((x, y), distance)) = queue.pop() {
        let tile = tiles[y][x];
        // theoretically not necessary
        if tile == Tile::Ground {
            continue;
        }

        for n in tile.possible_neighbors() {
            let (nx, ny) = (x as isize + n.0, y as isize + n.1);
            if nx < 0
                || ny < 0
                || ny >= tiles.len() as isize
                || nx >= tiles[ny as usize].len() as isize
            {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            let neighbor = tiles[ny][nx];
            if neighbor == Tile::Ground {
                continue;
            }

            let n_tile = tiles[ny][nx];
            if !n_tile.possible_neighbors().contains(&(-n.0, -n.1)) {
                continue;
            }

            let current_distance = distances[ny][nx];
            if current_distance.unwrap_or(isize::MAX) > distance + 1 {
                distances[ny][nx] = Some(distance + 1);
                queue.push(((nx, ny), distance + 1));
            }
        }
    }

    distances
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Ground,
    Start,
}

impl Tile {
    fn possible_neighbors(&self) -> Vec<(isize, isize)> {
        match self {
            Tile::Vertical => vec![(0, 1), (0, -1)],
            Tile::Horizontal => vec![(1, 0), (-1, 0)],
            Tile::TLBend => vec![(0, -1), (-1, 0)],
            Tile::TRBend => vec![(0, -1), (1, 0)],
            Tile::BLBend => vec![(0, 1), (-1, 0)],
            Tile::BRBend => vec![(0, 1), (1, 0)],
            Tile::Ground => panic!("neighbors of ground"),
            Tile::Start => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        }
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));

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

        let input = advent_of_code::template::read_file("inputs", DAY);
        assert_eq!(part_two(&input), Some(287));
    }
}
