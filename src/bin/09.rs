advent_of_code::solution!(9);
use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut histories = parse(input);

    let ans = histories
        .iter_mut()
        .filter_map(|h| {
            // this should never fail, but returning options allows us to use the ? operator
            h.last_mut()?.push(0);

            // iterating from bottom to top, where prev is underneath (later than) curr
            for (prev, curr) in (0..h.len()).rev().tuple_windows() {
                let prev_last = h[prev].last()?;
                let curr_last = h[curr].last()?;

                let new = prev_last + curr_last;
                h[curr].push(new);
            }

            // last number of earliest history
            Some(h.first()?.last()?)
        })
        .sum::<i32>();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = parse(input);

    // VecDeque bc we push_front which Vec doesn't support
    let mut histories = histories
        .into_iter()
        .map(|h| h.into_iter().map(VecDeque::from).collect_vec())
        .collect_vec();

    let ans = histories
        .iter_mut()
        // this should never fail, but returning options allows us to use the ? operator
        .filter_map(|h| {
            h.last_mut()?.push_back(0);

            // iterating from bottom to top, where prev is underneath (later than) curr
            for (prev, curr) in (0..h.len()).rev().tuple_windows() {
                // im lazy
                let prev_first = h[prev][0];
                let curr_first = h[curr][0];

                let new = curr_first - prev_first;
                h[curr].push_front(new);
            }

            // first number of earliest history
            Some(h.first_mut()?[0])
        })
        .sum::<i32>();

    Some(ans)
}

type History = Vec<i32>;
type Histories = Vec<History>;

/// returns a vector of histories fully simulated down the tree
fn parse(input: &str) -> Vec<Histories> {
    let mut histories = input
        .lines()
        .map(|l| {
            vec![l
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()]
        })
        .collect_vec();

    for h in histories.iter_mut() {
        for i in 0.. {
            let curr = &h[i];

            if curr.iter().all(|n| *n == 0) {
                break;
            }

            // i love itertools
            let next = curr
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();

            h.push(next);
        }
    }

    histories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
