#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<i64>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    input
        .iter()
        .cloned()
        .map(|mut curr| {
            let mut len = curr.len();
            let mut all_zeros = false;
            while !all_zeros {
                all_zeros = true;
                for i in 0..len - 1 {
                    curr[i] = curr[i + 1] - curr[i];
                    all_zeros &= curr[i] == 0;
                }
                len -= 1;
            }
            curr[len..].iter().sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    input
        .iter()
        .cloned()
        .map(|mut curr| {
            let mut start = 0;
            let mut all_zeros = false;
            while !all_zeros {
                all_zeros = true;
                for i in (start + 1..curr.len()).rev() {
                    curr[i] = curr[i] - curr[i - 1];
                    all_zeros &= curr[i] == 0;
                }
                start += 1;
            }
            curr[..start].iter().rev().fold(0, |acc, &n| n - acc)
        })
        .sum()
}
