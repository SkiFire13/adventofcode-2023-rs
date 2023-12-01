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
    input
        .iter()
        .map(|line| {
            let (lhspos, lhs) = (0..line.len())
                .find_map(|i| {
                    let line = &line[i..];
                    let b = line.as_bytes()[0];
                    Some(match () {
                        _ if b.is_ascii_digit() => (i + 1, (b - b'0') as u32),
                        _ if line.starts_with("one") => (i + 3, 1),
                        _ if line.starts_with("two") => (i + 3, 2),
                        _ if line.starts_with("three") => (i + 5, 3),
                        _ if line.starts_with("four") => (i + 4, 4),
                        _ if line.starts_with("five") => (i + 4, 5),
                        _ if line.starts_with("six") => (i + 3, 6),
                        _ if line.starts_with("seven") => (i + 5, 7),
                        _ if line.starts_with("eight") => (i + 5, 8),
                        _ if line.starts_with("nine") => (i + 4, 9),
                        _ => return None,
                    })
                })
                .unwrap();
            let line = &line[lhspos..];
            let rhs = (0..line.len())
                .find_map(|i| {
                    let line = &line[..line.len() - i];
                    let b = line.as_bytes()[line.len() - 1];
                    Some(match () {
                        _ if b.is_ascii_digit() => (b - b'0') as u32,
                        _ if line.ends_with("one") => 1,
                        _ if line.ends_with("two") => 2,
                        _ if line.ends_with("three") => 3,
                        _ if line.ends_with("four") => 4,
                        _ if line.ends_with("five") => 5,
                        _ if line.ends_with("six") => 6,
                        _ if line.ends_with("seven") => 7,
                        _ if line.ends_with("eight") => 8,
                        _ if line.ends_with("nine") => 9,
                        _ => return None,
                    })
                })
                .unwrap_or(lhs);

            lhs * 10 + rhs
        })
        .sum()
}
