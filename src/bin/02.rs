advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

type Game = Vec<Set>;

pub fn part_one(input: &str) -> Option<usize> {
    let games = parse(input);

    let mut ans = 0;
    for (i, game) in games.into_iter().enumerate() {
        if game
            .into_iter()
            .any(|s| s.red > 12 || s.green > 13 || s.blue > 14)
        {
            continue;
        }

        ans += i + 1;
    }

    Some(ans as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games = parse(input);

    let mut ans = 0;
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for s in game {
            red = red.max(s.red);
            green = green.max(s.green);
            blue = blue.max(s.blue);
        }

        let power = red * green * blue;

        ans += power
    }

    Some(ans as usize)
}

fn parse(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.trim().lines() {
        let (_, important) = line.split_once(":").unwrap();
        let important = important.trim();

        let game = important
            .split(";")
            .map(|set| {
                let mut s = Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                set.trim().split(", ").for_each(|cubes| {
                    let (value, color) = cubes.split_once(" ").unwrap();
                    let value = value.parse::<usize>().unwrap();
                    let color = match color {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => panic!("Unknown color"),
                    };
                    match color {
                        Color::Red => s.red = value as usize,
                        Color::Green => s.green = value as usize,
                        Color::Blue => s.blue = value as usize,
                    }
                });
                s
            })
            .collect::<Vec<_>>();

        games.push(game);
    }

    games
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
