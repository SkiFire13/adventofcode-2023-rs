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
            let lhs = line.bytes().find(u8::is_ascii_digit).unwrap();
            let rhs = line.bytes().rfind(u8::is_ascii_digit).unwrap();
            (lhs - b'0') as u32 * 10 + (rhs - b'0') as u32
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    const DIGIT_NAMES: [(&[u8], u32); 9] = [
        (b"one", 1),
        (b"two", 2),
        (b"three", 3),
        (b"four", 4),
        (b"five", 5),
        (b"six", 6),
        (b"seven", 7),
        (b"eight", 8),
        (b"nine", 9),
    ];

    fn find_digit<T>(mut f: impl FnMut(&[u8], u32) -> Option<T>) -> Option<T> {
        DIGIT_NAMES.into_iter().find_map(|(d, n)| f(d, n))
    }

    input
        .iter()
        .map(|line| {
            let line = line.as_bytes();

            let (rest, lhs) = std::iter::successors(Some(line), |line| Some(line.split_first()?.1))
                .find_map(|line| match line.first() {
                    Some(&b @ b'1'..=b'9') => Some((&line[1..], (b - b'0') as u32)),
                    _ => find_digit(|d, n| line.strip_prefix(d).map(|rest| (rest, n))),
                })
                .unwrap();

            let rhs = std::iter::successors(Some(rest), |line| Some(line.split_last()?.1))
                .find_map(|line| match line.last() {
                    Some(&b @ b'1'..=b'9') => Some((b - b'0') as u32),
                    _ => find_digit(|d, n| line.ends_with(d).then(|| n)),
                })
                .unwrap_or(lhs);

            lhs * 10 + rhs
        })
        .sum()
}
