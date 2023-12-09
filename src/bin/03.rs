use std::collections::HashMap;

fn is_part_number(x: usize, y: usize, length: usize, schema: &Vec<Vec<char>>) -> bool {
    let start_x: usize = isize::max(
        isize::try_from(x).unwrap() - isize::try_from(length).unwrap() - 1,
        0,
    )
    .try_into()
    .unwrap();

    for i in -1..=1 {
        if (y == 0 && i == -1) || (y == schema.len() - 1 && i == 1) {
            continue;
        }

        let found = schema[usize::try_from(isize::try_from(y).unwrap() + i).unwrap()]
            [start_x..usize::min(x + 1, schema[y].len() - 1)]
            .iter()
            .any(|e| !e.is_ascii_digit() && *e != '.');

        if found {
            return true;
        }
    }

    false
}

fn solve_part_1(input: &str) -> u64 {
    let schema: Vec<Vec<char>> = input
        .lines()
        .map(|e| e.chars().collect::<Vec<char>>())
        .collect();

    let mut number = String::new();
    let mut result = 0;

    for (y, line) in schema.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            match *symbol {
                '0'..='9' => {
                    number.push(*symbol);

                    if x == line.len() - 1 {
                        if is_part_number(x, y, number.len(), &schema) {
                            result += number.parse::<u64>().unwrap();
                        }

                        number.clear();
                    }
                }
                _ => {
                    if !number.is_empty() {
                        if is_part_number(x, y, number.len(), &schema) {
                            result += number.parse::<u64>().unwrap();
                        }

                        number.clear();
                    }
                }
            }
        }
    }

    result
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type Gear = Vec<u64>;

fn populate_gears(
    x: usize,
    y: usize,
    length: usize,
    schema: &Vec<Vec<char>>,
    gears: &mut HashMap<Location, Gear>,
    number: u64,
) {
    let start_x: usize = isize::max(
        isize::try_from(x).unwrap() - isize::try_from(length).unwrap() - 1,
        0,
    )
    .try_into()
    .unwrap();

    for i in -1..=1 {
        if (y == 0 && i == -1) || (y == schema.len() - 1 && i == 1) {
            continue;
        }

        let s = &schema[usize::try_from(isize::try_from(y).unwrap() + i).unwrap()]
            [start_x..usize::min(x + 1, schema[y].len() - 1)];
        let pos = s.iter().position(|&e| e == '*');

        if let Some(gear_x) = pos {
            if let Some(gear) = gears.get_mut(&Location::new(
                gear_x + start_x,
                usize::try_from(isize::try_from(y).unwrap() + i).unwrap(),
            )) {
                gear.push(number);
            } else {
                gears.insert(
                    Location::new(
                        gear_x + start_x,
                        usize::try_from(isize::try_from(y).unwrap() + i).unwrap(),
                    ),
                    vec![number],
                );
            }
        }
    }
}

fn solve_part_2(input: &str) -> u64 {
    let schema: Vec<Vec<char>> = input
        .lines()
        .map(|e| e.chars().collect::<Vec<char>>())
        .collect();

    let mut number = String::new();
    let mut gears: HashMap<Location, Gear> = HashMap::new();

    for (y, line) in schema.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            match *symbol {
                '0'..='9' => {
                    number.push(*symbol);

                    if x == line.len() - 1 {
                        populate_gears(
                            x,
                            y,
                            number.len(),
                            &schema,
                            &mut gears,
                            number.parse().unwrap(),
                        );

                        number.clear();
                    }
                }
                _ => {
                    if !number.is_empty() {
                        populate_gears(
                            x,
                            y,
                            number.len(),
                            &schema,
                            &mut gears,
                            number.parse().unwrap(),
                        );
                        number.clear();
                    }
                }
            }
        }
    }

    gears
        .values()
        .filter(|gear| gear.len() == 2)
        .map(|gear| gear.iter().product::<u64>())
        .sum()
}

fn main() {
    let input = include_str!("../../data/input/03.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day03_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/03.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 467835);
    }
}
