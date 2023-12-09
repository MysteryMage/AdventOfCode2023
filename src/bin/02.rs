use std::str::FromStr;

#[derive(Debug)]
struct ParsingTurnError;

#[derive(Debug)]
struct Turn {
    red: u64,
    green: u64,
    blue: u64,
}

impl Turn {
    fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }
}

impl FromStr for Turn {
    type Err = ParsingTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut turn = Turn::new(0, 0, 0);

        for e in s.split(',') {
            let mut it = e.split(' ');
            it.next();
            let amount = it
                .next()
                .expect("Should be the amount of colored cube")
                .parse::<u64>()
                .expect("Should be the number");

            match it.next().expect("Should be a color") {
                "blue" => turn.blue = amount,
                "green" => turn.green = amount,
                "red" => turn.red = amount,
                _ => panic!("Not a valid color"),
            }
        }

        Ok(turn)
    }
}

fn solve_part_1(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for (i, line) in input.lines().enumerate() {
        let result = line
            .split(':')
            .nth(1)
            .expect("Should be the game's turns")
            .split(';')
            .map(|e| e.parse::<Turn>().expect("Expected a turn format"))
            .find(|e| e.red > 12 || e.green > 13 || e.blue > 14);

        if result.is_none() {
            sum +=
                u64::try_from(i).expect("Number should not be larger than 32 bit unsigned int") + 1;
        }
    }

    sum
}

fn solve_part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut max_amount = (0, 0, 0);

        line.split(':')
            .nth(1)
            .expect("Should be the game's turns")
            .split(';')
            .map(|e| e.parse::<Turn>().expect("Expected a turn format"))
            .for_each(|turn| {
                if turn.red > max_amount.0 {
                    max_amount.0 = turn.red;
                }

                if turn.green > max_amount.1 {
                    max_amount.1 = turn.green;
                }

                if turn.blue > max_amount.2 {
                    max_amount.2 = turn.blue;
                }
            });

        sum += max_amount.0 * max_amount.1 * max_amount.2;
    }

    sum
}

fn main() {
    let input = include_str!("../../data/input/02.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day02_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/02.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 2286);
    }
}
