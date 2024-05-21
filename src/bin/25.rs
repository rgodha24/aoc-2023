advent_of_code::solution!(25);
use advent_of_code::helpers::*;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn parse<'a>(input: &'a str) -> Graph<&'a str> {
    let nodes = input
        .lines()
        .map(|l| l.split_once(':').unwrap().0)
        .collect_vec();
    let edges = input
        .lines()
        .map(|l| {
            let (node, conns) = l.split_once(':').unwrap();

            conns.trim().split_whitespace().map(move |c| (node, c))
        })
        .flatten()
        .collect_vec();

    let mut graph = Graph::new(nodes);

    for (from, to) in edges {
        // for some reason, we can sometimes only learn about a node in the `to` field
        graph.add_node(to);

        graph.add_undirected_edge(&from, &to);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
