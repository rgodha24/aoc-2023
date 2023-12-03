use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    num: isize,
    start_x: isize,
    end_x: isize,
    y: isize,
    found: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = vec![];
    let mut symbols = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();

        while let Some((x, c)) = iter.next() {
            let mut num = 0;
            if let Some(n) = c.to_digit(10) {
                num = n;
                while matches!(iter.peek(), Some((_, c)) if c.is_digit(10)) {
                    let (_, c) = iter.next().unwrap();
                    num = num * 10 + c.to_digit(10).unwrap();
                }
            } else if c != '.' {
                symbols.push((x, y));
            }

            if num != 0 {
                let end_x = x + num.to_string().len() - 1;

                parts.push(Part {
                    num: num as isize,
                    start_x: x as isize,
                    end_x: end_x as isize,
                    y: y as isize,
                    found: false,
                });
            }
        }
    }

    println!("{:?}", parts);
    println!("{:?}", symbols);

    for (x, y) in symbols {
        for (dx, dy) in itertools::iproduct!(-1..=1, -1..=1) {
            let x = (x as isize + dx);
            let y = (y as isize + dy);
            for part in &mut parts {
                if y == part.y && (part.start_x..=part.end_x).contains(&x) {
                    part.found = true;
                    break;
                }
            }
        }
    }

    let mut sum = 0;
    for part in parts {
        if part.found {
            sum += part.num as u32;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = vec![];
    let mut symbols = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut iter = line.chars().enumerate().peekable();

        while let Some((x, c)) = iter.next() {
            let mut num = 0;
            if let Some(n) = c.to_digit(10) {
                num = n;
                while matches!(iter.peek(), Some((_, c)) if c.is_digit(10)) {
                    let (_, c) = iter.next().unwrap();
                    num = num * 10 + c.to_digit(10).unwrap();
                }
            } else if c == '*' {
                symbols.push((x, y));
            }

            if num != 0 {
                let end_x = x + num.to_string().len() - 1;

                parts.push(Part {
                    num: num as isize,
                    start_x: x as isize,
                    end_x: end_x as isize,
                    y: y as isize,
                    found: false,
                });
            }
        }
    }

    let mut sum = 0;
    for (x, y) in symbols {
        let mut gears = HashSet::new();
        for (dx, dy) in itertools::iproduct!(-1..=1, -1..=1) {
            let x = x as isize + dx;
            let y = y as isize + dy;
            for part in &mut parts {
                if y == part.y && (part.start_x..=part.end_x).contains(&x) {
                    part.found = true;

                    gears.insert(part.num);
                    break;
                }
            }
        }

        println!("{:?}", gears);

        if gears.len() == 2 {
            let gears: Vec<_> = gears.into_iter().collect();
            sum += gears[0] * gears[1];
        }
    }

    Some(sum as u32)
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
