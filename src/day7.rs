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

fn kind(mut hand: [u8; 5]) -> Kind {
    hand.sort();

    let mut counts = HashMap::<u8, u32>::new();
    for b in hand {
        *counts.entry(b).or_default() += 1;
    }

    let mut lcounts = counts.values().copied().collect::<Vec<_>>();
    lcounts.sort();

    if lcounts == [5] {
        return Kind::Five;
    }
    if lcounts == [1, 4] {
        return Kind::Four;
    }
    if lcounts == [2, 3] {
        return Kind::Full;
    }
    if lcounts == [1, 1, 3] {
        return Kind::Three;
    }
    if lcounts == [1, 2, 2] {
        return Kind::Two;
    }
    if lcounts == [1, 1, 1, 2] {
        return Kind::One;
    }
    if lcounts == [1, 1, 1, 1, 1] {
        return Kind::High;
    }

    panic!()
}

fn n(card: u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        _ => card - b'0',
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|&(cards, bid)| (kind(cards), cards.map(n), cards, bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, _, _, bid))| (i + 1) * bid)
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind2 {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

fn kind2(mut hand: [u8; 5]) -> Kind2 {
    hand.sort();

    let mut counts = HashMap::<u8, u32>::new();
    for b in hand {
        *counts.entry(b).or_default() += 1;
    }

    let countj = *counts.get(&b'J').unwrap_or(&0);

    let mut lcounts = counts.values().copied().collect::<Vec<_>>();
    lcounts.sort();

    if lcounts == [5] {
        return Kind2::Five;
    }
    if lcounts == [1, 4] && countj != 0 {
        return Kind2::Five;
    }
    if lcounts == [1, 4] {
        return Kind2::Four;
    }
    if lcounts == [2, 3] && countj != 0 {
        return Kind2::Five;
    }
    if lcounts == [2, 3] {
        return Kind2::Full;
    }
    if lcounts == [1, 1, 3] && countj != 0 {
        return Kind2::Four;
    }
    if lcounts == [1, 1, 3] {
        return Kind2::Three;
    }
    if lcounts == [1, 2, 2] && countj == 2 {
        return Kind2::Four;
    }
    if lcounts == [1, 2, 2] && countj == 1 {
        return Kind2::Full;
    }
    if lcounts == [1, 2, 2] {
        return Kind2::Two;
    }
    if lcounts == [1, 1, 1, 2] && countj != 0 {
        return Kind2::Three;
    }
    if lcounts == [1, 1, 1, 2] {
        return Kind2::One;
    }
    if lcounts == [1, 1, 1, 1, 1] && countj != 0 {
        return Kind2::One;
    }
    if lcounts == [1, 1, 1, 1, 1] {
        return Kind2::High;
    }

    panic!()
}

fn n2(card: u8) -> u8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'T' => 10,
        b'0'..=b'9' => card - b'0',
        b'J' => 0,
        _ => panic!(),
    }
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|&(cards, bid)| (kind2(cards), cards.map(n2), cards, bid))
        .sorted()
        .enumerate()
        .map(|(i, (_, _, _, bid))| (i + 1) * bid)
        .sum()
}
