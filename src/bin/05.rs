advent_of_code::solution!(5);
use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (mut seeds, maps) = parse(input);

    for i in 0..=6 {
        for seed in seeds.iter_mut() {
            *seed = maps[i].dest_of(*seed);
        }
    }

    seeds.into_iter().min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (seeds, maps) = parse(input);
    let mut seeds = seeds
        .into_iter()
        .tuple_windows()
        .enumerate()
        // filter on evens because we only want every other tuple (theres definitely a better way
        // of doing this that im missing)
        // we transform the start, range into (start, end) tuples
        .filter_map(|(i, (a, b))| if i % 2 != 0 { None } else { Some((a, a + b)) })
        .collect::<Vec<_>>();

    // might be O(n^4) lmao
    for i in 0..=6 {
        seeds.reverse();

        let mut new_seeds = Vec::new();
        let map = &maps[i];

        while let Some((start, end)) = seeds.pop() {
            for (range_start, range_end) in map.ranges() {
                let overlap_start = start.max(range_start);
                let overlap_end = end.min(range_end);

                // range doesn't overlap, so try a new one
                if overlap_start >= overlap_end {
                    continue;
                }

                new_seeds.push((map.dest_of(overlap_start), map.dest_of(overlap_end)));

                // now, we make sure that the range we chose fully contains start..end, and if it
                // doesnt, then we add the missing parts to the seeds list to find the ranges for
                // those
                if overlap_start > start {
                    // end is non inclusive
                    seeds.push((start, overlap_start));
                }
                if overlap_end < end {
                    // end is non inclusive
                    seeds.push((overlap_end, end));
                }

                // we already found the range that start..end is in, so we can break
                break;
            }
        }

        seeds = new_seeds;
    }
    let seeds: HashSet<usize> = seeds.into_iter().map(|(a, _)| a).collect();
    let mut seeds = seeds.into_iter().collect::<Vec<_>>();
    seeds.sort();
    println!("locations: {:?}", &seeds[0..5]);

    // this doesn't work for the real thing for some reason??
    // on my real input, the answer was seeds[2] LMAO
    seeds.first().cloned()
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<MapEntry>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct MapEntry {
    dest_start: usize,
    source_start: usize,
    range: usize,
}

impl MapEntry {
    fn dest_of(&self, num: usize) -> Option<usize> {
        if num < self.source_start || num >= (self.source_start + self.range) {
            return None;
        }
        Some(self.dest_start + (num - self.source_start))
    }
    /// not inclusive
    fn end(&self) -> usize {
        self.source_start + self.range
    }
}

impl Ord for MapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_start.cmp(&other.source_start)
    }
}

impl PartialOrd for MapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn dest_of(&self, num: usize) -> usize {
        for entry in &self.entries {
            if let Some(dest) = entry.dest_of(num) {
                return dest;
            }
        }

        num
    }

    /// start inclusive, end not inclusive
    fn ranges(&self) -> Vec<(usize, usize)> {
        let mut ranges = Vec::new();

        for (a, b) in self.entries.iter().tuple_windows() {
            ranges.push((a.source_start, a.end()));
            ranges.push((a.end(), b.source_start));
        }

        // we dont include the last range's source_start to end, so do that here.
        let last = self.entries.last().unwrap();
        ranges.push((last.source_start, last.end()));
        // and just in case
        ranges.push((last.end(), usize::MAX));

        if ranges[0].0 != 0 {
            ranges.insert(0, (0, ranges[0].0));
        }

        ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

        ranges.into_iter().filter(|(a, b)| a != b).collect()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        for line in s.lines() {
            let [dest_start, source_start, range] = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| ())?;

            entries.push(MapEntry {
                dest_start,
                source_start,
                range,
            });
        }

        entries.sort();

        Ok(Map { entries })
    }
}

fn parse(input_str: &str) -> (Vec<usize>, Vec<Map>) {
    let double_lines = input_str.split("\n\n").collect::<Vec<_>>();

    let (_, seeds) = double_lines[0].split_once(": ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut maps = Vec::new();

    for m in double_lines[1..].iter() {
        let (_, map) = m.split_once("\n").unwrap();
        maps.push(map.parse().unwrap());
    }

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
