advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
            .map(|v| v.first().unwrap() * 10 + v.last().unwrap())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_whitespace()
            .map(|s| {
                let mut ans = vec![];
                for i in 0..s.len() {
                    // really wish I remembered this during the actual advent of code :(
                    // the as_bytes is just so we don't clone for no reason
                    // and the b'x' is a character auto-converted to a byte
                    let n = match s.as_bytes()[i..] {
                        [c, ..] if (c as char).is_digit(10) => (c as char).to_digit(10),
                        [b'o', b'n', b'e', ..] => Some(1),
                        [b't', b'w', b'o', ..] => Some(2),
                        [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
                        [b'f', b'o', b'u', b'r', ..] => Some(4),
                        [b'f', b'i', b'v', b'e', ..] => Some(5),
                        [b's', b'i', b'x', ..] => Some(6),
                        [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
                        [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
                        [b'n', b'i', b'n', b'e', ..] => Some(9),
                        _ => None,
                    };

                    if let Some(n) = n {
                        ans.push(n);
                    }
                }

                let first = *ans.first().unwrap();
                let last = *ans.last().unwrap();

                (first, last)
            })
            .map(|(f, l)| f * 10 + l)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // different than given value bc i changed input to second one
        assert_eq!(result, Some(242));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
