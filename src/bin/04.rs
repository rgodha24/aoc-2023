advent_of_code::solution!(4);
use std::collections::{BTreeMap, HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for (winning, ours) in parse(input) {
        let amt = ours
            .into_iter()
            .filter(|o| winning.contains(o))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });

        total += amt;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let mut cards = input
        .into_iter()
        .map(|(winning, ours)| {
            let num_add = ours.into_iter().filter(|o| winning.contains(o)).count();
            // num add is the amount of cards after i that we should add to
            // amt is the amount of the card we have
            (num_add, 1)
        })
        .collect_vec();

    let mut count = 0;
    let mut i = 0;

    // while loop here so we don't have an immutable borrow (from the iterator)
    // and mutable borrow (cards[j + 1].1 += amt) at the same time
    while i < cards.len() {
        let (num_add, amt) = cards[i];
        count += amt;
        // for all of the cards after this, add amt to them
        // this is the same as iterating over this card id multiple times, just much faster.
        for j in i..(i + num_add) {
            // j+1 because we don't want to add to the current card
            cards[j + 1].1 += amt;
        }
        i += 1;
    }

    Some(count)
}

fn parse(input: &str) -> Vec<(HashSet<u32>, Vec<u32>)> {
    let mut cards = Vec::new();

    for input in input.lines() {
        let (_, input) = input.split_once(":").unwrap();
        let (left, right) = input.split_once("|").unwrap();

        let winning: HashSet<_> = left
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let ours = right
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();
        cards.push((winning, ours));
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
