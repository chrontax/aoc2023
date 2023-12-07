use std::{cmp::Ordering, str::FromStr};

fn main() {
    let now = std::time::Instant::now();
    let result = part1(INPUT);
    println!("Part 1: {} [{:?}]", result, now.elapsed());
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
        Some(self.value().cmp(&other.value()))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[test]
fn test_card_cmp() {
    assert_eq!(Card::N(2).cmp(&Card::N(2)), Ordering::Equal);
    assert_eq!(Card::N(2).cmp(&Card::N(3)), Ordering::Less);
    assert_eq!(Card::N(3).cmp(&Card::N(2)), Ordering::Greater);
    assert_eq!(Card::N(2).cmp(&Card::T), Ordering::Less);
    assert_eq!(Card::T.cmp(&Card::N(2)), Ordering::Greater);
    assert_eq!(Card::T.cmp(&Card::T), Ordering::Equal);
    assert_eq!(Card::T.cmp(&Card::J), Ordering::Less);
    assert_eq!(Card::J.cmp(&Card::T), Ordering::Greater);
    assert_eq!(Card::J.cmp(&Card::J), Ordering::Equal);
    assert_eq!(Card::J.cmp(&Card::Q), Ordering::Less);
    assert_eq!(Card::Q.cmp(&Card::J), Ordering::Greater);
    assert_eq!(Card::Q.cmp(&Card::Q), Ordering::Equal);
    assert_eq!(Card::Q.cmp(&Card::K), Ordering::Less);
    assert_eq!(Card::K.cmp(&Card::Q), Ordering::Greater);
    assert_eq!(Card::K.cmp(&Card::K), Ordering::Equal);
    assert_eq!(Card::K.cmp(&Card::A), Ordering::Less);
    assert_eq!(Card::A.cmp(&Card::K), Ordering::Greater);
    assert_eq!(Card::A.cmp(&Card::A), Ordering::Equal);
}

impl Card {
    fn value(&self) -> u8 {
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

#[test]
fn test_card_value() {
    assert_eq!(Card::N(2).value(), 0);
    assert_eq!(Card::N(3).value(), 1);
    assert_eq!(Card::N(4).value(), 2);
    assert_eq!(Card::N(5).value(), 3);
    assert_eq!(Card::N(6).value(), 4);
    assert_eq!(Card::N(7).value(), 5);
    assert_eq!(Card::N(8).value(), 6);
    assert_eq!(Card::N(9).value(), 7);
    assert_eq!(Card::T.value(), 8);
    assert_eq!(Card::J.value(), 9);
    assert_eq!(Card::Q.value(), 10);
    assert_eq!(Card::K.value(), 11);
    assert_eq!(Card::A.value(), 12);
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

#[test]
fn test_card_from() {
    assert_eq!(Card::from('2'), Card::N(2));
    assert_eq!(Card::from('3'), Card::N(3));
    assert_eq!(Card::from('4'), Card::N(4));
    assert_eq!(Card::from('5'), Card::N(5));
    assert_eq!(Card::from('6'), Card::N(6));
    assert_eq!(Card::from('7'), Card::N(7));
    assert_eq!(Card::from('8'), Card::N(8));
    assert_eq!(Card::from('9'), Card::N(9));
    assert_eq!(Card::from('T'), Card::T);
    assert_eq!(Card::from('J'), Card::J);
    assert_eq!(Card::from('Q'), Card::Q);
    assert_eq!(Card::from('K'), Card::K);
    assert_eq!(Card::from('A'), Card::A);
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

#[test]
fn test_hand_type_cmp() {
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::FiveOfAKind),
        Ordering::Equal
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::FourOfAKind),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::FullHouse),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::ThreeOfAKind),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::TwoPair),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::OnePair),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FiveOfAKind.cmp(&HandType::HighCard),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::FiveOfAKind),
        Ordering::Less
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::FourOfAKind),
        Ordering::Equal
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::FullHouse),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::ThreeOfAKind),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::TwoPair),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::OnePair),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FourOfAKind.cmp(&HandType::HighCard),
        Ordering::Greater
    );
    assert_eq!(
        HandType::FullHouse.cmp(&HandType::FiveOfAKind),
        Ordering::Less
    );
    assert_eq!(
        HandType::FullHouse.cmp(&HandType::FourOfAKind),
        Ordering::Less
    );
    assert_eq!(
        HandType::FullHouse.cmp(&HandType::FullHouse),
        Ordering::Equal
    );
}

impl From<[Card; 5]> for HandType {
    fn from(hand: [Card; 5]) -> Self {
        let mut counts = [0; 13];
        for card in hand.iter() {
            counts[card.value() as usize] += 1;
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

#[test]
fn test_hand_type_from() {
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(2)
        ]),
        HandType::FiveOfAKind
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(3)
        ]),
        HandType::FourOfAKind
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(3),
            Card::N(3)
        ]),
        HandType::FullHouse
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(2),
            Card::N(3),
            Card::N(4)
        ]),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(3),
            Card::N(3),
            Card::N(4)
        ]),
        HandType::TwoPair
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(2),
            Card::N(3),
            Card::N(4),
            Card::N(5)
        ]),
        HandType::OnePair
    );
    assert_eq!(
        HandType::from([
            Card::N(2),
            Card::N(3),
            Card::N(4),
            Card::N(5),
            Card::N(6)
        ]),
        HandType::HighCard
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .fold(([Card::A; 5], 0), |mut acc, c| {
                acc.0[acc.1] = Card::from(c);
                acc.1 += 1;
                acc
            })
            .0;

        Ok(Self {
            cards,
            hand_type: HandType::from(cards),
        })
    }
}

#[test]
fn test_hand_from_str() {
    assert_eq!(
        Hand::from_str("22222"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(2), Card::N(2), Card::N(2), Card::N(2)],
            hand_type: HandType::FiveOfAKind
        })
    );
    assert_eq!(
        Hand::from_str("22223"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(2), Card::N(2), Card::N(2), Card::N(3)],
            hand_type: HandType::FourOfAKind
        })
    );
    assert_eq!(
        Hand::from_str("22333"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(2), Card::N(3), Card::N(3), Card::N(3)],
            hand_type: HandType::FullHouse
        })
    );
    assert_eq!(
        Hand::from_str("22334"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(2), Card::N(3), Card::N(3), Card::N(4)],
            hand_type: HandType::TwoPair
        })
    );
    assert_eq!(
        Hand::from_str("22345"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(2), Card::N(3), Card::N(4), Card::N(5)],
            hand_type: HandType::OnePair
        })
    );
    assert_eq!(
        Hand::from_str("25346"),
        Ok(Hand {
            cards: [Card::N(2), Card::N(5), Card::N(3), Card::N(4), Card::N(6)],
            hand_type: HandType::HighCard
        })
    );
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

#[test]
fn test_hand_cmp() {
    assert_eq!(
        Hand::from_str("22222").unwrap().cmp(&Hand::from_str("22222").unwrap()),
        Ordering::Equal
    );
    assert_eq!(
        Hand::from_str("22222").unwrap().cmp(&Hand::from_str("22223").unwrap()),
        Ordering::Greater
    );
    assert_eq!(
        Hand::from_str("22223").unwrap().cmp(&Hand::from_str("22222").unwrap()),
        Ordering::Less
    );
    assert_eq!(
        Hand::from_str("22223").unwrap().cmp(&Hand::from_str("22233").unwrap()),
        Ordering::Greater
    );
    assert_eq!(
        Hand::from_str("22233").unwrap().cmp(&Hand::from_str("22223").unwrap()),
        Ordering::Less
    );
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

#[test]
fn test_parse() {
    assert_eq!(
        parse("22233 3"),
        vec![
            (
                Hand {
                    cards: [Card::N(2), Card::N(2), Card::N(2), Card::N(3), Card::N(3)],
                    hand_type: HandType::FullHouse
                },
                3
            )
        ]
    );
}

fn part1(input: &str) -> u32 {
    let mut hands = parse(input);
    hands.sort_by(|(a, _), (b, _)| a.cmp(b));
    hands.iter().map(|(_, value)| *value).enumerate().fold(
        0,
        |acc, (i, value)| acc + (value * (i as u32 + 1)),
    )
}
