advent_of_code::solution!(6);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let (first, second) = input.split_once("\n").unwrap();
    let (_, time) = first.split_once(":").unwrap();
    let time = time
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();
    let (_, distance) = second.split_once(":").unwrap();
    let distance = distance
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec();

    Some(
        time.into_iter()
            .zip(distance.into_iter())
            // do this for every single (time, distance) combination
            .map(|(t, d)| {
                (0..t)
                    // the distance that we go using time n to charge up
                    // the speed we get is `n`, and the time we go at that speed is
                    // `(t - n)`, which is how we get `n * (t - n)`
                    .map(|n| n * (t - n))
                    // make sure we beat the goal distance
                    .filter(|&n| n > d)
                    .count()
            })
            // I love that rust has this function built in
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (first, second) = input.split_once("\n").unwrap();
    let (_, time) = first.split_once(":").unwrap();
    let time = time.replace(" ", "").parse::<usize>().unwrap();
    let distance = second
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    Some((0..time).filter(|&n| n * (time - n) > distance).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
