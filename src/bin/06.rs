struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    fn new(time: u64, distance_record: u64) -> Self {
        Self {
            time,
            distance_record,
        }
    }

    fn get_possible_solutions(&self) -> u64 {
        let mut start = 0;
        let mut end = 0;

        for i in 1..self.time {
            if (self.time - i) * i > self.distance_record {
                end = i;
            }
        }

        for i in (1..self.time).rev() {
            if (self.time - i) * i > self.distance_record {
                start = i;
            }
        }

        end - start + 1
    }
}

fn parse_line_to_vec(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect::<Vec<u64>>()
}

fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let (time, distance) = (lines.next().unwrap(), lines.next().unwrap());

    let time = parse_line_to_vec(time);
    let distance = parse_line_to_vec(distance);

    let mut races = Vec::new();

    for i in 0..time.len() {
        races.push(Race::new(time[i], distance[i]));
    }

    races
        .iter()
        .map(Race::get_possible_solutions)
        .product::<u64>()
}

fn parse_line_to_value(line: &str) -> u64 {
    line.split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|e| !e.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let (time, distance) = (lines.next().unwrap(), lines.next().unwrap());

    let time = parse_line_to_value(time);
    let distance = parse_line_to_value(distance);
    let race = Race::new(time, distance);

    race.get_possible_solutions()
}

fn main() {
    let input = include_str!("../../data/input/06.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day06_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/06.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 71503);
    }
}
