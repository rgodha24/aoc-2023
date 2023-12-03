use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Part {
    num: u32,
    start_x: isize,
    end_x: isize,
    y: isize,
}

#[derive(Debug)]
enum Symbol {
    Gear,
    Other,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (parts, symbols) = parse(input);

    let mut ans = HashSet::new();
    for (x, y, _) in symbols {
        for (dx, dy) in itertools::iproduct!(-1..=1, -1..=1) {
            let x = x + dx;
            let y = y + dy;
            for part in &parts {
                if y == part.y && (part.start_x..=part.end_x).contains(&x) {
                    ans.insert(part.num);
                    break;
                }
            }
        }
    }

    Some(ans.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (parts, symbols) = parse(input);

    let symbols = symbols
        .into_iter()
        .filter(|(_, _, s)| matches!(s, Symbol::Gear))
        .collect_vec();

    let mut sum = 0;
    for (x, y, _) in symbols {
        let mut gears = HashSet::new();
        for (dx, dy) in itertools::iproduct!(-1..=1, -1..=1) {
            let x = x + dx;
            let y = y + dy;
            for part in &parts {
                if y == part.y && (part.start_x..=part.end_x).contains(&x) {
                    gears.insert(part.num);
                    break;
                }
            }
        }

        if gears.len() == 2 {
            let gears: Vec<_> = gears.into_iter().collect();
            sum += gears[0] * gears[1];
        }
    }

    Some(sum)
}

fn parse(input: &str) -> (Vec<Part>, Vec<(isize, isize, Symbol)>) {
    let mut parts = vec![];
    let mut symbols = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();

        while let Some((x, c)) = iter.next() {
            match c {
                c if c.is_digit(10) => {
                    let mut num = c.to_digit(10).unwrap();
                    let mut end_x = x;
                    while matches!(iter.peek(), Some((_, c)) if c.is_digit(10)) {
                        let (_, c) = iter.next().unwrap();
                        num = num * 10 + c.to_digit(10).unwrap();
                        end_x += 1;
                    }

                    parts.push(Part {
                        num: num as u32,
                        start_x: x as isize,
                        end_x: end_x as isize,
                        y: y as isize,
                    });
                }
                '.' => {}
                '*' => symbols.push((x as isize, y as isize, Symbol::Gear)),
                _ => symbols.push((x as isize, y as isize, Symbol::Other)),
            }
        }
    }

    (parts, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
