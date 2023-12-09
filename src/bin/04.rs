use std::str::FromStr;

#[derive(Debug)]
struct ParsingError;

struct Card {
    winning_numbers: Vec<u64>,
    numbers: Vec<u64>,
}

impl Card {
    fn new(winning_numbers: Vec<u64>, numbers: Vec<u64>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn get_winning_count(&self) -> u64 {
        let mut count = 0;

        for win in &self.winning_numbers {
            if self.numbers.contains(win) {
                count += 1;
            }
        }

        count
    }

    fn get_points(&self) -> u64 {
        let count = self.get_winning_count();

        if count == 0 {
            return 0;
        }

        2_u64.pow((count - 1).try_into().unwrap())
    }

    fn set_card_winning_copies(&self, cards: &mut [u64], id: usize) {
        let count = self.get_winning_count();

        for _ in 0..cards[id] {
            for card in cards
                .iter_mut()
                .take(id + usize::try_from(count).unwrap() + 1)
                .skip(id + 1)
            {
                *card += 1;
            }
        }
    }
}

impl FromStr for Card {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('|').map(|e| {
            e.split(' ')
                .filter(|e| !e.is_empty())
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        });

        Ok(Card::new(it.next().unwrap(), it.next().unwrap()))
    }
}

fn solve_part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|e| {
            e.split(':')
                .nth(1)
                .unwrap()
                .parse::<Card>()
                .unwrap()
                .get_points()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let mut cards: Vec<u64> = vec![1; input.lines().collect::<Vec<&str>>().len()];

    input.lines().enumerate().for_each(|(i, e)| {
        e.split(':')
            .nth(1)
            .unwrap()
            .parse::<Card>()
            .unwrap()
            .set_card_winning_copies(&mut cards, i);
    });

    cards.iter().sum()
}

fn main() {
    let input = include_str!("../../data/input/04.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day04_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/04.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 30);
    }
}
