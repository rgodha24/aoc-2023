advent_of_code::solution!(15);
use std::num::Wrapping;

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(|x| hash(x) as usize).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = vec![vec![]; 256];

    input.trim().split(',').for_each(|s| match s.as_bytes() {
        // rust pattern matching >>>>
        [label @ .., b'-'] => {
            // safety: we know that the label is valid utf8
            let label = unsafe { std::str::from_utf8_unchecked(label) };
            let index = hash(label) as usize;

            if state[index].iter().find(|(x, _)| x == &label).is_some() {
                state[index].retain(|(x, _)| x != &label);
            }
        }
        [label @ .., b'=', c] => {
            // safety: we know that the label is valid utf8
            let label = unsafe { std::str::from_utf8_unchecked(label) };
            let index = hash(label) as usize;
            let value = *c - b'0';

            if let Some(i) = state[index].iter().position(|(x, _)| x == &label) {
                state[index][i] = (label, value);
            } else {
                state[index].push((label, value));
            }
        }
        _ => panic!("Invalid input"),
    });

    let mut ans = 0;
    for i in 0..256 {
        for (j, (_, n)) in state[i].iter().enumerate() {
            ans += (i + 1) * (j + 1) * (*n as usize);
        }
    }

    Some(ans)
}

fn hash(input: &str) -> u8 {
    let mut ans = Wrapping(0);
    input.as_bytes().iter().for_each(|x| {
        ans += Wrapping(*x);
        ans *= Wrapping(17);
    });

    ans.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
    //     rn=1 becomes 30.
    // cm- becomes 253.
    // qp=3 becomes 97.
    // cm=2 becomes 47.
    // qp- becomes 14.
    // pc=4 becomes 180.
    // ot=9 becomes 9.
    // ab=5 becomes 197.
    // pc- becomes 48.
    // pc=6 becomes 214.
    // ot=7 becomes 231.

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    #[case("HASH", 52)]
    fn test_hash(#[case] input: &str, #[case] expected: u8) {
        assert_eq!(hash(input), expected);
    }
}



