#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(FxHashSet<u32>, FxHashSet<u32>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (win, got) = line.split_once(" | ").unwrap();
            let win = win
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<FxHashSet<_>>();
            let got = got
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<FxHashSet<_>>();
            (win, got)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(win, got)| (1 << win.intersection(got).count()) >> 1)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut counts = vec![1; input.len()];

    for (i, (win, got)) in input.iter().enumerate() {
        for j in 0..win.intersection(got).count() {
            counts[i + 1 + j] += counts[i];
        }
    }

    counts.iter().sum()
}
