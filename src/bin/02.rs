advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    println!("{input}");
    let lines = input.trim().lines();
    let parsed = lines
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .trim()
                .split(";")
                .map(|s| {
                    s.trim()
                        .split(", ")
                        .map(|s| {
                            let (first, second) = s.split_once(" ").unwrap();
                            let first = first.parse::<u32>().unwrap();
                            let second = match second {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                _ => panic!("Unknown color"),
                            };
                            (first, second)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let singles = parsed
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|s| {
                    let mut single = Single {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for (value, color) in s {
                        match color {
                            Color::Red => single.red = value as usize,
                            Color::Green => single.green = value as usize,
                            Color::Blue => single.blue = value as usize,
                        }
                    }
                    single
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, single) in singles.into_iter().enumerate() {
        if single
            .into_iter()
            .any(|s| s.red > 12 || s.green > 13 || s.blue > 14)
        {
            continue;
        }

        println!("{i}");
        ans += i + 1;
    }

    Some(ans as u32)
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct Single {
    red: usize,
    green: usize,
    blue: usize,
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("{input}");
    let lines = input.trim().lines();
    let parsed = lines
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .trim()
                .split(";")
                .map(|s| {
                    s.trim()
                        .split(", ")
                        .map(|s| {
                            let (first, second) = s.split_once(" ").unwrap();
                            let first = first.parse::<u32>().unwrap();
                            let second = match second {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                _ => panic!("Unknown color"),
                            };
                            (first, second)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let singles = parsed
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|s| {
                    let mut single = Single {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for (value, color) in s {
                        match color {
                            Color::Red => single.red = value as usize,
                            Color::Green => single.green = value as usize,
                            Color::Blue => single.blue = value as usize,
                        }
                    }
                    single
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, single) in singles.into_iter().enumerate() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for s in single {
            red = red.max(s.red);
            green = green.max(s.green);
            blue = blue.max(s.blue);
        }

        let power = red * green * blue;

        ans += power
    }

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
