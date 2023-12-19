advent_of_code::solution!(19);
use std::{
    collections::HashMap,
    ops::{Index, IndexMut, RangeInclusive},
};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, parts) = parse(input);
    let mut ans = 0;

    for p in parts {
        let mut loc = "in";
        loop {
            let rule = &rules[loc];
            let new_loc = rule
                .rules
                .iter()
                .find_map(|r| {
                    let num = p[r.cat];
                    let res = match r.op {
                        Op::LessThan => num < r.num,
                        Op::GreaterThan => num > r.num,
                    };

                    res.then_some(&r.to)
                })
                .unwrap_or(&rule.end);

            match new_loc {
                Location::Accept => {
                    ans += p.x + p.m + p.a + p.s;
                    break;
                }
                Location::Reject => {
                    break;
                }
                Location::Rule(new) => loc = new,
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, _) = parse(input);
    let range = Ranges {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    Some(count(range, &rules, Location::Rule("in")))
}

// shoutout hyperneutrino https://www.youtube.com/watch?v=3RwIpUegdU4
// no way i do this on my own
fn count(mut range: Ranges, rules: &HashMap<&str, Rule>, loc: Location) -> u64 {
    let loc = match loc {
        Location::Reject => return 0,
        Location::Accept => return range.count(),
        Location::Rule(loc) => loc,
    };

    let rule = &rules[loc];

    let mut total = 0;
    for r in &rule.rules {
        let low = *range[r.cat].start();
        let high = *range[r.cat].end();
        let n = r.num as u64;
        let (t, f) = match r.op {
            Op::LessThan => (low..=(n - 1), n..=high),
            Op::GreaterThan => ((n + 1)..=high, low..=n),
        };

        if !t.is_empty() {
            let mut nr = range.clone();
            nr[r.cat] = t;
            total += count(nr, rules, r.to.clone());
        }
        if f.is_empty() {
            break;
        } else {
            // continue following the rules with this
            range[r.cat] = f;
        }
    }

    total += count(range, rules, rule.end.clone());

    total
}

// parse function from hell wtf
fn parse<'a>(input: &'a str) -> (HashMap<&'a str, Rule<'a>>, impl Iterator<Item = Part> + 'a) {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let parts = parts.trim().lines().map(|l| {
        // remove {}
        let l = &l[1..l.len() - 1];
        let (x, m, a, s) = l
            .split(',')
            .map(|p| p.split_once('=').unwrap().1)
            .map(|p| p.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Part { x, m, a, s }
    });

    let rules = rules
        .trim()
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').unwrap();
            // remove }
            let rules = &rules[0..(rules.len() - 1)];
            let mut rules = rules.split(',').collect_vec();
            let last = rules.pop().unwrap();
            let last = match last {
                "R" => Location::Reject,
                "A" => Location::Accept,
                _ => Location::Rule(last),
            };

            let rules = rules
                .into_iter()
                .map(|r| {
                    let mut chars = r.chars();
                    let cat = match chars.next().unwrap() {
                        'x' => Category::X,
                        'm' => Category::M,
                        'a' => Category::A,
                        's' => Category::S,
                        c => panic!("invalid category {c}"),
                    };
                    let op = match chars.next().unwrap() {
                        '<' => Op::LessThan,
                        '>' => Op::GreaterThan,
                        _ => panic!("invalid op"),
                    };
                    let (num, loc) = r[2..].split_once(':').unwrap();
                    let num = num.parse().unwrap();
                    let loc = match loc {
                        "R" => Location::Reject,
                        "A" => Location::Accept,
                        _ => Location::Rule(loc),
                    };

                    SingleRule {
                        cat,
                        op,
                        num,
                        to: loc,
                    }
                })
                .collect_vec();

            (name, Rule { rules, end: last })
        })
        .collect();

    (rules, parts)
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
enum Location<'a> {
    Rule(&'a str),
    Reject,
    Accept,
}

#[derive(Debug, Clone)]
struct SingleRule<'a> {
    cat: Category,
    op: Op,
    num: u32,
    to: Location<'a>,
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    rules: Vec<SingleRule<'a>>,
    /// none of the other rules resolved, so send it here
    end: Location<'a>,
}

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct Ranges {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl Index<Category> for Part {
    type Output = u32;

    fn index(&self, index: Category) -> &Self::Output {
        match index {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl Index<Category> for Ranges {
    type Output = RangeInclusive<u64>;

    fn index(&self, index: Category) -> &Self::Output {
        match index {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl IndexMut<Category> for Ranges {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        match index {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

impl Ranges {
    fn count(self) -> u64 {
        let x = self.x.end() - self.x.start() + 1;
        let m = self.m.end() - self.m.start() + 1;
        let a = self.a.end() - self.a.start() + 1;
        let s = self.s.end() - self.s.start() + 1;

        x * m * a * s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(432434));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
