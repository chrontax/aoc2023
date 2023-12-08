use std::{cmp::Ordering, str::FromStr};

use rayon::prelude::*;

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
    let now = std::time::Instant::now();
    let result = part2(INPUT);
    println!("Part 2: {} [{:?}]", result, now.elapsed());
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value(false).cmp(&other.value(false)))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Card {
    const COUNT: usize = 13;

    fn value(&self, joker: bool) -> u8 {
        if joker {
            if self == &Self::J {
                0
            } else {
                self.value(false) + 1
            }
        } else {
            match self {
                Self::N(n) => n - 2,
                Self::T => 8,
                Self::J => 9,
                Self::Q => 10,
                Self::K => 11,
                Self::A => 12,
            }
        }
    }

    fn from_value(value: u8) -> Self {
        match value {
            0 => Self::J,
            1 => Self::N(2),
            2 => Self::N(3),
            3 => Self::N(4),
            4 => Self::N(5),
            5 => Self::N(6),
            6 => Self::N(7),
            7 => Self::N(8),
            8 => Self::T,
            9 => Self::Q,
            10 => Self::K,
            11 => Self::A,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            n => Self::N(n.to_digit(10).unwrap() as u8),
        }
    }
}

fn parse_cards(input: &str) -> [Card; 5] {
    input
        .chars()
        .enumerate()
        .fold([Card::A; 5], |mut acc, (i, c)| {
            acc[i] = Card::from(c);
            acc
        })
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_with_joker(cards: [Card; 5]) -> Self {
        let joker_count = cards.iter().filter(|c| c == &&Card::J).count();
        let no_joker = cards
            .iter()
            .filter(|c| c != &&Card::J)
            .copied()
            .fold(([Card::A; 5], 0), |mut acc, card| {
                if card != Card::J {
                    acc.0[acc.1] = card;
                    acc.1 += 1;
                }
                acc
            })
            .0;
        if joker_count == 0 {
            Self::from(no_joker)
        } else {
            (1..(Card::COUNT - 1).pow(joker_count as u32))
                .into_par_iter()
                .map(|i| {
                    let mut no_joker = no_joker;
                    for j in 0..joker_count {
                        no_joker[4 - j] = Card::from_value(
                            ((i / (Card::COUNT - 1).pow(j as u32)) % (Card::COUNT - 1)) as u8,
                        );
                    }
                    Self::from(no_joker)
                })
                .map(|cards| Self::from(cards))
                .max()
                .unwrap()
        }
    }
}

impl From<[Card; 5]> for HandType {
    fn from(hand: [Card; 5]) -> Self {
        let mut counts = [0; 13];
        for card in hand.iter() {
            counts[card.value(false) as usize] += 1;
        }

        let mut pairs = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;

        for count in counts.iter() {
            match count {
                2 => pairs += 1,
                3 => three_of_a_kind = true,
                4 => four_of_a_kind = true,
                5 => five_of_a_kind = true,
                _ => (),
            }
        }

        if five_of_a_kind {
            Self::FiveOfAKind
        } else if four_of_a_kind {
            Self::FourOfAKind
        } else if three_of_a_kind && pairs == 1 {
            Self::FullHouse
        } else if three_of_a_kind {
            Self::ThreeOfAKind
        } else if pairs == 2 {
            Self::TwoPair
        } else if pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    joker: bool,
}

impl Hand {
    fn from_with_joker(cards: [Card; 5]) -> Self {
        Self {
            joker: true,
            cards,
            hand_type: HandType::from_with_joker(cards),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = parse_cards(s);

        Ok(Self {
            joker: false,
            cards,
            hand_type: HandType::from(cards),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_cmp = self.hand_type.cmp(&other.hand_type);
        if type_cmp.is_eq() {
            for i in 0..5 {
                let card_cmp = self.cards[i].cmp(&other.cards[i]);
                if card_cmp.is_gt() {
                    return Some(Ordering::Greater);
                } else if card_cmp.is_lt() {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        } else {
            Some(type_cmp)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = split.next().unwrap().parse().unwrap();
            let value = split.next().unwrap().parse().unwrap();
            (hand, value)
        })
        .collect()
}

fn parse_joker(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = Hand::from_with_joker(parse_cards(split.next().unwrap()));
            let value = split.next().unwrap().parse().unwrap();
            (hand, value)
        })
        .collect()
}

fn helper(mut hands: Vec<(Hand, u32)>) -> u32 {
    hands.sort_by(|(a, _), (b, _)| a.cmp(b));
    hands
        .iter()
        .map(|(_, value)| *value)
        .enumerate()
        .fold(0, |acc, (i, value)| acc + (value * (i as u32 + 1)))
}

fn part1(input: &str) -> u32 {
    helper(parse(input))
}

fn part2(input: &str) -> u32 {
    helper(parse_joker(input))
}
