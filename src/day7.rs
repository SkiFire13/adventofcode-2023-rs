#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<([u8; 5], usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let bytes = line[..5].as_bytes().try_into().unwrap();
            let bid = line[6..].parse().unwrap();
            (bytes, bid)
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn kind<const PART2: bool>(cards: [u8; 5]) -> Kind {
    let count = |card| cards.iter().filter(|&&c| c == card).count();

    let mut counts = cards.map(count);
    counts.sort_unstable();

    let countj = if PART2 { count(b'J') } else { 0 };

    match (counts, countj) {
        ([5, 5, 5, 5, 5], _) => Kind::Five,
        ([1, 4, 4, 4, 4], 0) => Kind::Four,
        ([1, 4, 4, 4, 4], _) => Kind::Five,
        ([2, 2, 3, 3, 3], 0) => Kind::Full,
        ([2, 2, 3, 3, 3], _) => Kind::Five,
        ([1, 1, 3, 3, 3], 0) => Kind::Three,
        ([1, 1, 3, 3, 3], _) => Kind::Four,
        ([1, 2, 2, 2, 2], 0) => Kind::Two,
        ([1, 2, 2, 2, 2], 1) => Kind::Full,
        ([1, 2, 2, 2, 2], 2) => Kind::Four,
        ([1, 1, 1, 2, 2], 0) => Kind::One,
        ([1, 1, 1, 2, 2], _) => Kind::Three,
        ([1, 1, 1, 1, 1], 0) => Kind::High,
        ([1, 1, 1, 1, 1], _) => Kind::One,
        _ => unreachable!(),
    }
}

fn n<const PART2: bool>(card: u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' if PART2 => 0,
        b'J' => 11,
        b'T' => 10,
        _ => card - b'0',
    }
}

fn solve<const PART2: bool>(input: &Input) -> usize {
    input
        .iter()
        .map(|&(cards, bid)| (kind::<PART2>(cards), cards.map(n::<PART2>), bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum()
}

pub fn part1(input: &Input) -> usize {
    solve::<false>(input)
}

pub fn part2(input: &Input) -> usize {
    solve::<true>(input)
}
