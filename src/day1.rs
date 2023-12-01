#[allow(unused_imports)]
use super::prelude::*;
type Input<'input> = Vec<&'input str>;

pub fn input_generator(input: &str) -> Input {
    input.lines().collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            (
                line.chars().filter(|c| c.is_digit(10)).next().unwrap() as u8 - b'0',
                line.chars().filter(|c| c.is_digit(10)).next_back().unwrap() as u8 - b'0',
            )
        })
        .map(|(d1, d2)| d1 as u32 * 10 + d2 as u32)
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let groups = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .iter()
        .map(|line| {
            let (_, lhs) = groups
                .iter()
                .filter_map(|(s, n)| Some((line.find(s)?, n)))
                .min()
                .unwrap();
            let (_, rhs) = groups
                .iter()
                .filter_map(|(s, n)| Some((line.rfind(s)?, n)))
                .max()
                .unwrap();
            (lhs, rhs)
        })
        .map(|(d1, d2)| d1 * 10 + d2)
        .sum()
}
