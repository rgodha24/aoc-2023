advent_of_code::solution!(12);
use advent_of_code::helpers::*;
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<Row>().unwrap())
            .map(|l| l.combinations())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<Row<Cache>>().unwrap())
            .map(|l| l.unfold())
            .map(|l| l.combinations())
            .sum(),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    /// .
    Operational,
    /// ?
    Unknown,
    /// #
    Damaged,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Row<C: Cacheable = NoCache> {
    tiles: Vec<Tile>,
    nums: Vec<usize>,
    /// cache for the function f
    /// can be either NoCache (part 1) or Cache (part 2)
    cache: C,
}

impl<C: Cacheable> Row<C> {
    fn combinations(mut self) -> u64 {
        self.f(0, 0, 0) as u64
    }

    // tile_index and num_index are exactly what you think
    // damaged count is the amount of damaged tiles we've seen so far
    // we do this instead of cloning around `Row` because this is a lot faster
    // and easier to put into the cache
    //
    // also i have no idea what to call this function LMAO
    fn f(&mut self, tile_index: usize, num_index: usize, damaged_count: usize) -> usize {
        // if it's in the cache, return it
        if let Some(result) = self.cache.get(&(tile_index, num_index, damaged_count)) {
            return result;
        }

        // if we're at the end of the tiles
        if tile_index == self.tiles.len() {
            if num_index == self.nums.len() && damaged_count == 0 {
                // all of the numbers have been placed before this, so this is a valid combination
                return 1;
            } else if num_index == self.nums.len() - 1 && self.nums[num_index] == damaged_count {
                // we're at the last number, and we have the correct number of damaged tiles
                // this prevents us from needing to add a '.' to the end of self.tiles
                return 1;
            } else {
                // we're at the end without the correct number of damaged tiles, so this is invalid
                return 0;
            }
        }

        let mut ans = 0;
        let tile = self.tiles[tile_index];

        match tile {
            Tile::Operational => {
                if damaged_count == 0 {
                    // we don't have any damaged tiles, so just keep going
                    ans += self.f(tile_index + 1, num_index, 0);
                }
                if num_index < self.nums.len() && damaged_count == self.nums[num_index] {
                    // we have the correct number of damaged tiles, so we increment num_index
                    ans += self.f(tile_index + 1, num_index + 1, 0);
                }
            }
            Tile::Unknown => {
                // we have an unknown tile, so we can either make it operational or damaged

                // operational
                if damaged_count == 0 {
                    // we don't have any damaged tiles, so we can make it operational
                    ans += self.f(tile_index + 1, num_index, 0);
                }
                if num_index < self.nums.len() && damaged_count == self.nums[num_index] {
                    ans += self.f(tile_index + 1, num_index + 1, 0);
                }

                // damaged
                ans += self.f(tile_index + 1, num_index, damaged_count + 1);
            }
            Tile::Damaged => {
                // we have a damaged tile, so we increment damaged_len
                ans += self.f(tile_index + 1, num_index, damaged_count + 1);
            }
        }

        self.cache
            .insert((tile_index, num_index, damaged_count), ans);

        ans
    }

    fn unfold(mut self) -> Self {
        self.tiles.push(Tile::Unknown);
        // -1 to remove extra ? at end
        let n = self.tiles.len() * 5 - 1;
        // cycle repeats the iterator forever, so we take n elements from it
        self.tiles = self.tiles.into_iter().cycle().take(n).collect_vec();

        // same thing but without the -1 because we don't join nums with anything, just repeat it
        let n = self.nums.len() * 5;
        self.nums = self.nums.into_iter().cycle().take(n).collect_vec();

        self
    }
}

impl<C: Cacheable> FromStr for Row<C> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(' ').ok_or(())?;

        let tiles = first
            .chars()
            .map(|c| match c {
                '.' => Tile::Operational,
                '?' => Tile::Unknown,
                '#' => Tile::Damaged,
                _ => panic!("Invalid tile"),
            })
            .collect_vec();

        let nums = second.split(',').map(|s| s.parse().unwrap()).collect_vec();

        Ok(Self {
            tiles,
            nums,
            cache: C::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // this library is so useful ty chris biscardi for using it in a yt video
    use rstest::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    // ty to aoc reddit for these test cases
    #[case(".##.?#??.#.?# 2,1,1,1", 1)]
    #[case("###.### 3", 0)]
    fn test_combinations(#[case] line: Row, #[case] expected: u64) {
        assert_eq!(line.combinations(), expected);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    #[case(".##.?#??.#.?# 2,1,1,1", 1)]
    #[case("###.### 3", 0)]
    fn test_combos_unfolded(#[case] mut line: Row<Cache>, #[case] expected: u64) {
        line = line.unfold();
        assert_eq!(line.combinations(), expected);
    }

    #[rstest]
    #[case(".# 1", ".#?.#?.#?.#?.# 1,1,1,1,1")]
    #[case(
        "???.### 1,1,3",
        "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
    )]
    fn test_unfolding(#[case] orig: Row, #[case] expected: Row) {
        let unfolded = orig.unfold();

        assert_eq!(unfolded, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
