advent_of_code::solution!(23);
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<isize> {
    let (grid, edges, start, end) = parse(input);

    let graph = build_graph(&edges, &grid, true);

    let ans = dfs(start, end, &grid, &mut HashSet::new(), &graph);

    Some(ans)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (grid, edges, start, end) = parse(input);

    let graph = build_graph(&edges, &grid, false);

    let ans = dfs(start, end, &grid, &mut HashSet::new(), &graph);

    Some(ans)
}

fn dfs(p: Point, end: Point, grid: &Grid<Tile>, seen: &mut HashSet<Point>, graph: &Graph) -> isize {
    if p == end {
        return 0;
    }

    let mut max = isize::MIN;

    seen.insert(p.clone());
    for nx in graph[&p].keys() {
        if seen.contains(nx) {
            continue;
        }

        let d = dfs(nx.clone(), end.clone(), grid, seen, graph) + graph[&p][nx] as isize;
        max = d.max(max);
    }
    seen.remove(&p);

    max
}

type Graph = HashMap<Point, HashMap<Point, usize>>;

fn build_graph(edges: &[Point], grid: &Grid<Tile>, is_part_one: bool) -> Graph {
    let mut graph: HashMap<_, _> = edges.iter().map(|p| (*p, HashMap::new())).collect();

    for p in edges {
        let mut seen: HashSet<_> = [p.clone()].into();
        let mut queue = vec![(0, p.clone())];

        while let Some((nd, np)) = queue.pop() {
            if nd > 0 && graph.contains_key(&np) {
                graph.get_mut(&p).unwrap().insert(np, nd);
                continue;
            }

            let neighbors = if is_part_one {
                match grid[np] {
                    Tile::Slope(d) => vec![np + &d],
                    Tile::Empty => grid.neighbors_of_filtered(np.clone(), |t, _| *t != Tile::Wall),
                    _ => continue,
                }
            } else {
                grid.neighbors_of_filtered(np.clone(), |t, _| *t != Tile::Wall)
            };

            for n in neighbors {
                if seen.contains(&n) {
                    continue;
                }

                seen.insert(n.clone());
                queue.push((nd + 1, n));
            }
        }
    }

    graph
}

fn parse(input: &str) -> (Grid<Tile>, Vec<Point>, Point, Point) {
    let grid: Grid<Tile> = Grid::from_chars(input);

    // all the points with 3 neighbors are graph edges bc they're the only points that have 2
    // options on where to go
    let mut edges = grid
        .flat_iter()
        .filter_map(|(t, p)| (*t == Tile::Empty).then_some(p))
        .filter(|p| {
            grid.neighbors_of_filtered(p.clone(), |t, _| *t != Tile::Wall)
                .len()
                >= 3
        })
        .collect_vec();

    // this is so ugly
    let start = grid
        .row(0)
        .into_iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::Empty)
        .unwrap()
        .0;
    let end = grid
        .row(grid.height() - 1)
        .into_iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::Empty)
        .unwrap()
        .0;

    let start = Point::new(start, 0);
    let end = Point::new(end, grid.height() - 1);

    edges.push(start);
    edges.push(end);

    (grid, edges, start, end)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            c => Self::Slope(c.into()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Wall => '#',
            Self::Empty => '.',
            Self::Slope(d) => d.clone().into(),
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
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
