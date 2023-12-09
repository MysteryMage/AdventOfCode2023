use std::str::FromStr;

#[derive(Debug)]
struct MapParsingError;

struct Map {
    source: Vec<u64>,
    dest: Vec<u64>,
    length: Vec<u64>,
}

impl Map {
    fn get(&self, number: u64) -> u64 {
        for i in 0..self.source.len() {
            if (self.source[i]..self.source[i] + self.length[i]).contains(&number) {
                return self.dest[i] + (number - self.source[i]);
            }
        }

        number
    }
}

impl FromStr for Map {
    type Err = MapParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut source = Vec::new();
        let mut dest = Vec::new();
        let mut length = Vec::new();

        let mut it = s.split('\n').filter(|e| !e.is_empty());
        let _header = it.next();

        for line in it {
            let cur_map = line
                .split_whitespace()
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            dest.push(cur_map[0]);
            source.push(cur_map[1]);
            length.push(cur_map[2]);
        }

        Ok(Map {
            source,
            dest,
            length,
        })
    }
}

fn solve_part_1(input: &str) -> u64 {
    let mut almanac = input.split("\n\n");

    let seeds: Vec<u64> = almanac
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    let maps: Vec<Map> = almanac.map(|e| e.parse::<Map>().unwrap()).collect();

    seeds
        .iter()
        .map(|seed| {
            let mut cur_value = seed.to_owned();

            for e in &maps {
                cur_value = e.get(cur_value);
            }

            cur_value
        })
        .min()
        .unwrap()
}

fn solve_part_2(input: &str) -> u64 {
    let mut almanac = input.split("\n\n");

    let seeds_data: Vec<u64> = almanac
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    let mut seeds = Vec::new();

    let mut seed_start = 0;
    for (i, seed) in seeds_data.iter().enumerate() {
        if i % 2 == 0 {
            seed_start = *seed;
        } else {
            seeds.push(seed_start..seed_start + seed);
        }
    }

    let maps: Vec<Map> = almanac.map(|e| e.parse::<Map>().unwrap()).collect();
    let mut min: u64 = u64::MAX;

    for seed_range in seeds {
        for seed in seed_range {
            let mut cur_value = seed;

            for e in &maps {
                cur_value = e.get(cur_value);
            }

            if cur_value < min {
                min = cur_value;
            }
        }
    }

    min
}

fn main() {
    let input = include_str!("../../data/input/05.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day05_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/05.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 46);
    }
}
