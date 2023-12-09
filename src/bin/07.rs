advent_of_code::solution!(7);
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use std::{cmp::Ordering, str::FromStr};

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, |_| {}))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, |c| c.replace_jacks()))
}

fn solve(input: &str, edit_cards: impl Fn(&mut Cards)) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, rank) = line.split_once(' ').unwrap();
            let mut cards = hand.parse().unwrap();
            edit_cards(&mut cards);

            let rank: usize = rank.parse().unwrap();
            let hand = Hand::new(cards);

            (hand, rank)
        })
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(i, (_, rank))| (i + 1) * rank)
        .sum()
}

impl Rank {
    fn new(cards: &Cards) -> Self {
        let (most, second_most) = cards.most_common();

        let mut rank = match (most, second_most) {
            (5, _) => Rank::Five,
            (4, _) => Rank::Four,
            (3, 2) => Rank::Full,
            (3, _) => Rank::Three,
            (2, 2) => Rank::TwoPair,
            (2, _) => Rank::OnePair,
            (1, _) => Rank::High,
            _ => Rank::None,
        };

        for _ in 0..cards.jokers() {
            rank = rank.next();
        }

        rank
    }

    fn next(self) -> Self {
        match self {
            Rank::Five | Rank::Full => panic!(">5 cards in hand"),
            Rank::Four => Rank::Five,
            Rank::Three => Rank::Four,
            Rank::TwoPair => Rank::Full,
            Rank::OnePair => Rank::Three,
            Rank::High => Rank::OnePair,
            Rank::None => Rank::High,
        }
    }

    // theoretically should be an `impl Ord for Rank`
    // but this is less boilerplate
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl Hand {
    fn new(cards: Cards) -> Self {
        let rank = Rank::new(&cards);

        Self { cards, rank }
    }

    // theoretically should be an `impl Ord for Hand`
    // but this is less boilerplate
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank
            .cmp(&other.rank)
            .then(self.cards.tiebrake(&other.cards))
    }
}

impl Cards {
    fn replace_jacks(&mut self) {
        self.iter_mut().for_each(|c| {
            if *c == Card::J {
                *c = Card::Joker;
            }
        })
    }

    /// tiebrakes by iterating over cards and comparing them in order from start to finish
    fn tiebrake(&self, other: &Self) -> Ordering {
        self.iter()
            .map(|c| *c as u8)
            .zip(other.iter().map(|c| *c as u8))
            .find_map(|(a, b)| match a.cmp(&b) {
                Ordering::Equal => None,
                x => Some(x),
            })
            .unwrap_or(Ordering::Equal)
    }

    /// returns instances of (most_common, second_most_common)
    fn most_common(&self) -> (usize, usize) {
        let mut v = [0; 14];

        self.iter().for_each(|c| {
            if *c == Card::Joker {
                return;
            }
            v[*c as usize] += 1;
        });

        v.sort();

        (v[13], v[12])
    }

    fn jokers(&self) -> usize {
        self.iter().filter(|c| **c == Card::Joker).count()
    }
}

impl From<char> for Card {
    fn from(s: char) -> Self {
        match s {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card"),
        }
    }
}

impl FromStr for Cards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| c.into())
            .collect_vec()
            .try_into()
            .unwrap();

        Ok(Self(cards))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Cards,
    rank: Rank,
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
struct Cards([Card; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
    Joker = 0,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Rank {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
    /// JJJJJ needs this
    None = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
