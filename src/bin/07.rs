use std::hash::Hash;
use std::{collections::HashMap, str::FromStr};

trait Cardable {
    fn from_char(c: char) -> Self;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum Card {
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

impl Cardable for Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Not a valid card"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
enum CardWithJoker {
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
    Queen,
    King,
    Ace,
}

impl Cardable for CardWithJoker {
    fn from_char(c: char) -> Self {
        match c {
            'J' => CardWithJoker::Joker,
            '2' => CardWithJoker::Two,
            '3' => CardWithJoker::Three,
            '4' => CardWithJoker::Four,
            '5' => CardWithJoker::Five,
            '6' => CardWithJoker::Six,
            '7' => CardWithJoker::Seven,
            '8' => CardWithJoker::Eight,
            '9' => CardWithJoker::Nine,
            'T' => CardWithJoker::Ten,
            'Q' => CardWithJoker::Queen,
            'K' => CardWithJoker::King,
            'A' => CardWithJoker::Ace,
            _ => panic!("Not a valid card"),
        }
    }
}

type Hand<T> = [T; 5];

fn get_card_count<T>(arr: [T; 5]) -> HashMap<T, u64>
where
    T: Cardable,
    T: Eq,
    T: Hash,
    T: Copy,
{
    let mut counts: HashMap<T, u64> = HashMap::new();

    for card in arr {
        if counts.contains_key(&card) {
            counts.insert(card, counts.get(&card).unwrap() + 1);
        } else {
            counts.insert(card, 1);
        }
    }

    counts
}

#[derive(Debug)]
struct HandTypeParseErr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType<T>
where
    T: Cardable,
{
    HighCard(Hand<T>),
    OnePair(Hand<T>),
    TwoPair(Hand<T>),
    ThreeOfKind(Hand<T>),
    FullHouse(Hand<T>),
    FourOfKind(Hand<T>),
    FiveOfKind(Hand<T>),
}

fn from_cards<T>(max_count: u64, hand: Hand<T>, actual_hand: Hand<T>) -> HandType<T>
where
    T: std::cmp::PartialEq,
    T: Cardable,
    T: Copy,
{
    match max_count {
        5 => HandType::FiveOfKind(actual_hand),
        4 => HandType::FourOfKind(actual_hand),
        3 => {
            for c in &hand {
                if hand.iter().filter(|e| *e == c).count() == 2 {
                    return HandType::FullHouse(actual_hand);
                }
            }

            HandType::ThreeOfKind(actual_hand)
        }
        2 => {
            let mut prev_two_count: Option<T> = None;

            for cur_card in &hand {
                if hand.iter().filter(|e| *e == cur_card).count() == 2 {
                    if prev_two_count.is_none() {
                        prev_two_count = Some(*cur_card);
                    } else if prev_two_count.unwrap() != *cur_card {
                        return HandType::TwoPair(actual_hand);
                    }
                }
            }

            HandType::OnePair(actual_hand)
        }
        _ => HandType::HighCard(actual_hand),
    }
}

impl FromStr for HandType<Card> {
    type Err = HandTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand: [Card; 5] = [Card::Two; 5];

        for (i, c) in s.chars().enumerate() {
            hand[i] = Card::from_char(c);
        }

        Ok(from_cards(
            *get_card_count(hand).values().max().unwrap(),
            hand,
            hand,
        ))
    }
}

impl FromStr for HandType<CardWithJoker> {
    type Err = HandTypeParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand: [CardWithJoker; 5] = [CardWithJoker::Two; 5];

        for (i, c) in s.chars().enumerate() {
            hand[i] = CardWithJoker::from_char(c);
        }

        let hand_with_joker = hand;

        let counts = get_card_count(hand);
        let mut max_count = *counts.values().max().unwrap();

        if counts.get(&CardWithJoker::Joker).is_some() {
            let mut counts_vec: Vec<(&CardWithJoker, &u64)> = counts.iter().collect();
            counts_vec.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));

            if *counts_vec.last().unwrap().0 == CardWithJoker::Joker {
                counts_vec.pop();
            }

            if max_count != 5 {
                for (i, card) in hand_with_joker.iter().enumerate() {
                    if *card == CardWithJoker::Joker {
                        hand[i] = *counts_vec.last().unwrap().0;
                    }
                }
            }

            max_count = *get_card_count(hand).values().max().unwrap();
        }

        Ok(from_cards(max_count, hand, hand_with_joker))
    }
}

fn solve_part_1(input: &str) -> u64 {
    let mut hands: Vec<(HandType<Card>, u64)> = input
        .lines()
        .map(|e| {
            let mut it = e.split_whitespace();

            let hand: HandType<Card> = it.next().unwrap().parse().unwrap();
            let bid: u64 = it.next().unwrap().parse().unwrap();

            (hand, bid)
        })
        .collect();

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, e)| u64::try_from(i + 1).unwrap() * e.1)
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let mut hands: Vec<(HandType<CardWithJoker>, u64)> = input
        .lines()
        .map(|e| {
            let mut it = e.split_whitespace();

            let hand: HandType<CardWithJoker> = it.next().unwrap().parse().unwrap();
            let bid: u64 = it.next().unwrap().parse().unwrap();

            (hand, bid)
        })
        .collect();

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    hands
        .iter()
        .enumerate()
        .map(|(i, e)| u64::try_from(i + 1).unwrap() * e.1)
        .sum()
}

fn main() {
    let input = include_str!("../../data/input/07.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day07_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/07.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 5905);
    }
}
