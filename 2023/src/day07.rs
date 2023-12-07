use std::{cmp::Reverse, collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn evaluate_type(hand: &Hand) -> Self {
        let mut occurences = hand
            .0
            .iter()
            .fold(HashMap::<&Card, usize>::new(), |mut acc, cur| {
                *acc.entry(cur).or_default() += 1;
                acc
            });

        let jokers = occurences.remove(&Card::Joker).unwrap_or_default();
        if jokers == 5 {
            return Self::FiveOfKind;
        }

        let mut occurences = occurences.into_iter().collect::<Vec<_>>();
        occurences.sort_by_key(|c| Reverse(c.1));

        occurences[0].1 += jokers;

        match occurences.len() {
            1 => Self::FiveOfKind,
            2 => {
                if occurences[0].1 == 4 {
                    Self::FourOfKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if occurences[0].1 == 3 {
                    Self::ThreeOfKind
                } else {
                    Self::TwoPairs
                }
            }
            _ => {
                if occurences[0].1 == 2 {
                    Self::OnePair
                } else {
                    Self::HighCard
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand(Vec<Card>);

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = value.chars().map(Card::from).collect::<Vec<_>>();

        assert_eq!(cards.len(), 5);

        Self(cards)
    }
}

#[derive(Debug)]
struct EvaluatedHand {
    hand: Hand,
    hand_type: HandType,
}

impl Hand {
    fn evaluate(self) -> EvaluatedHand {
        let hand_type = HandType::evaluate_type(&self);

        EvaluatedHand {
            hand: self,
            hand_type,
        }
    }

    fn use_jokers(mut self) -> Self {
        self.0 = self
            .0
            .into_iter()
            .map(|card| match card {
                Card::Jack => Card::Joker,
                c => c,
            })
            .collect();
        self
    }
}

#[derive(Debug)]
struct Game(Vec<(Hand, u32)>);

impl Game {
    fn play(self, with_jokers: bool) -> u32 {
        let mut hands = self
            .0
            .into_iter()
            .map(|(hand, bid)| {
                let hand = if with_jokers { hand.use_jokers() } else { hand };
                (hand, bid)
            })
            .map(|(hand, bid)| (hand.evaluate(), bid))
            .collect::<Vec<_>>();
        hands.sort_by(|a, b| {
            let by_hand_type = a.0.hand_type.cmp(&b.0.hand_type);
            let by_cards = a.0.hand.cmp(&b.0.hand);
            by_hand_type.then(by_cards)
        });

        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (rank, (_, bid))| acc + bid * (rank as u32 + 1))
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let hands = value
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let hand = Hand::from(hand);
                let bid = bid.parse().unwrap();
                (hand, bid)
            })
            .collect();
        Self(hands)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let game = Game::from(input);

    game.play(false)
}

fn solve_second_part(input: &str) -> u32 {
    let game = Game::from(input);

    game.play(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_first_part() {
        let answer = 6440;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 5905;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_calculate_type_without_jokers() {
        assert_eq!(
            Hand::from("AAAAA").evaluate().hand_type,
            HandType::FiveOfKind
        );
        assert_eq!(
            Hand::from("AA8AA").evaluate().hand_type,
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::from("23332").evaluate().hand_type,
            HandType::FullHouse
        );
        assert_eq!(
            Hand::from("TTT98").evaluate().hand_type,
            HandType::ThreeOfKind
        );
        assert_eq!(Hand::from("23432").evaluate().hand_type, HandType::TwoPairs);
        assert_eq!(Hand::from("A23A4").evaluate().hand_type, HandType::OnePair);
        assert_eq!(Hand::from("23456").evaluate().hand_type, HandType::HighCard);
    }

    #[test]
    fn test_calculate_type_with_jokers() {
        assert_eq!(
            Hand::from("QJJQ2").use_jokers().evaluate().hand_type,
            HandType::FourOfKind
        );
    }

    check_answers!(250474325, 248909434);
}
