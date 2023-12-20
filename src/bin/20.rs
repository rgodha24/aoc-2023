advent_of_code::solution!(20);
use std::{collections::HashMap, fmt::Display};

use advent_of_code::helpers::*;
use itertools::Itertools;
use ModuleType::*;
use OnOff::*;
use Pulse::*;

pub fn part_one(input: &str) -> Option<u64> {
    let (mut map, broadcast_to) = parse(input);
    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut pulses = broadcast_to
            .iter()
            .map(|&dest| (dest, Pulse::Low, "broadcaster"))
            .collect_vec();

        // button -> broadcaster
        low_count += 1;

        while !pulses.is_empty() {
            let mut new_pulses = Vec::new();
            for (_, p, _) in &pulses {
                match p {
                    Low => low_count += 1,
                    High => high_count += 1,
                }
            }

            for (to, p, from) in pulses {
                // println!("{} -{} -> {}", from, p, to);

                if to == "output" || to == "rx" {
                    continue;
                }

                let m = map.get_mut(to).unwrap();

                if let Some(p) = m.accept(p, from) {
                    new_pulses.extend(m.dests.iter().map(|&dest| (dest, p, to)));
                }
            }

            pulses = new_pulses;
        }

        // println!("low: {}, high: {}", low_count, high_count);
    }

    Some(low_count * high_count)
}

// ok so basically, the module that sends to rx (dh in my case) is a Conjunction module
// Conjunction modules output low only when all inputs are high
// We assume that inputs to the conjunction module cycle, and take the lcm of all of those cycle
// lengths, which will give us the cycle length of the conjunction module that feeds rx, which is
// the answer to part 2
//
// https://www.youtube.com/watch?v=lxm6i21O83k hyperneutrino is the goat again
pub fn part_two(input: &str) -> Option<u64> {
    let (mut map, broadcast_to) = parse(input);

    let rx_sender = *map.iter().find(|(_, m)| m.dests.contains(&"rx")).unwrap().0;

    let mut seen: HashMap<&str, u64> = HashMap::new();
    let mut cycle_lens: HashMap<&str, u64> = HashMap::new();

    'outer: for i in 1.. {
        let mut pulses = broadcast_to
            .iter()
            .map(|&dest| (dest, Pulse::Low, "broadcaster"))
            .collect_vec();

        while !pulses.is_empty() {
            let mut new_pulses = Vec::new();

            for (to, p, from) in pulses {
                if to == "output" || to == "rx" {
                    continue;
                }

                if to == rx_sender && p == High {
                    *seen.entry(from).or_insert(0) += 1;
                    let cl = *cycle_lens.entry(from).or_insert(i);
                    let s = seen[from];
                    assert_eq!(i, s * cl, "cycling actually works");

                    if seen.values().all(|v| *v > 1) {
                        // i kinda hate this syntax ngl
                        break 'outer;
                    }
                }

                let m = map.get_mut(to).unwrap();

                if let Some(p) = m.accept(p, from) {
                    new_pulses.extend(m.dests.iter().map(|&dest| (dest, p, to)));
                }
            }

            pulses = new_pulses;
        }
    }

    Some(math::lcm(cycle_lens.values().copied()))
}

impl<'a> Module<'a> {
    fn accept(&mut self, pulse: Pulse, from: &'a str) -> Option<Pulse> {
        match (&mut self.mod_type, pulse) {
            (FlipFlop(_), High) => None,
            (FlipFlop(Off), Low) => {
                self.mod_type = FlipFlop(On);
                Some(High)
            }
            (FlipFlop(On), Low) => {
                self.mod_type = FlipFlop(Off);
                Some(Low)
            }
            (Conjunction(map), pulse) => {
                map.insert(from, pulse);
                if map.values().all(|p| matches!(p, High)) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum OnOff {
    On,
    #[default]
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType<'a> {
    FlipFlop(OnOff),
    Conjunction(HashMap<&'a str, Pulse>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module<'a> {
    mod_type: ModuleType<'a>,
    dests: Vec<&'a str>,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => write!(f, "low"),
            Pulse::High => write!(f, "high"),
        }
    }
}

/// returns the map of all modules, and the destinations of the `broadcaster` module
fn parse<'a>(input: &'a str) -> (HashMap<&'a str, Module<'a>>, Vec<&'a str>) {
    let mut map: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(" -> ").unwrap();
            let v = v.split(", ").collect_vec();

            (k, v)
        })
        .collect();

    let broadcast_to = map.remove("broadcaster").unwrap();

    let mut map: HashMap<_, _> = map
        .into_iter()
        .map(|(k, v)| {
            let mod_type = match k.chars().nth(0).unwrap() {
                '%' => ModuleType::FlipFlop(Default::default()),
                '&' => ModuleType::Conjunction(Default::default()),
                _ => panic!("unknown module type"),
            };
            let k = &k[1..];
            let v = Module { mod_type, dests: v };

            (k, v)
        })
        .collect();

    // this is the only clone :(
    for (k, m) in map.clone().into_iter() {
        for dest in &m.dests {
            if dest == &"output" || dest == &"rx" {
                continue;
            }

            let dest = map.get_mut(dest).unwrap();

            match &mut dest.mod_type {
                Conjunction(map) => {
                    map.insert(k, Pulse::Low);
                }
                FlipFlop(_) => {}
            }
        }
    }

    (map, broadcast_to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#,
        32000000
    )]
    #[case(
        r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#,
        11687500
    )]
    fn test_part_one(#[case] input: &str, #[case] expected: u64) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        // they don't give us a test case for part 2
    }
}
