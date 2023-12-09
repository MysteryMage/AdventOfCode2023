use std::collections::HashMap;

fn solve_part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>();
            let mut result = String::new();
            result.push(numbers.chars().next().unwrap());
            result.push(numbers.chars().last().unwrap());
            result.parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

fn word_to_num(line: &str, num_map: &HashMap<&str, char>) -> String {
    let lower_char_bound = 3;
    let upper_char_bound = 5;

    let mut result = String::new();

    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            result.push(c);
            continue;
        }

        for j in lower_char_bound..=upper_char_bound {
            if i + j <= line.len() {
                if let Some(num) = num_map.get(&line[i..i + j]) {
                    result.push(*num);
                }
            }
        }
    }

    result
}

fn solve_part_2(input: &str) -> u64 {
    let num_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    input
        .lines()
        .map(|line| {
            let numbers = word_to_num(line, &num_map);
            let mut result = String::new();
            result.push(numbers.chars().next().unwrap());
            result.push(numbers.chars().last().unwrap());
            result.parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../../data/input/01.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day01_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE1: &str = include_str!("../../data/sample/01_1.txt");
    const SAMPLE2: &str = include_str!("../../data/sample/01_2.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE1);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE2);
        assert_eq!(result, 281);
    }
}
