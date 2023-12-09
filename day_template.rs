fn solve_part_1(input: &str) -> u64 {
    todo!();
}

fn solve_part_2(input: &str) -> u64 {
    todo!();
}

fn main() {
    let input = include_str!("../../data/input/day_input.txt");

    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day_test {
    use crate::{solve_part_1, solve_part_2};

    const SAMPLE: &str = include_str!("../../data/sample/day_input.txt");

    #[test]
    fn part_1() {
        let result = solve_part_1(SAMPLE);
        assert_eq!(result, 0);
    }

    #[test]
    fn part_2() {
        let result = solve_part_2(SAMPLE);
        assert_eq!(result, 0);
    }
}
