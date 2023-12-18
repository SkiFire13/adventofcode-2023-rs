#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u8, usize, [u8; 6])>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let dir = line.as_bytes()[0];
            let (len, color) = line[2..].split_once(' ').unwrap();
            let len = len.parse().unwrap();
            let color = color[2..8].as_bytes().try_into().unwrap();
            (dir, len, color)
        })
        .collect()
}

fn solve(instructions: impl Iterator<Item = (u8, usize)>) -> usize {
    let mut area = 0;
    let mut perimeter = 0;
    let mut y = 0;

    for (dir, len) in instructions {
        match dir {
            b'R' => area += len as isize * y,
            b'L' => area -= len as isize * y,
            b'U' => y += len as isize,
            b'D' => y -= len as isize,
            _ => panic!(),
        }
        perimeter += len;
    }

    area.abs() as usize + perimeter / 2 + 1
}

pub fn part1(input: &Input) -> usize {
    solve(input.iter().map(|&(dir, len, _)| (dir, len)))
}

pub fn part2(input: &Input) -> usize {
    let hex_to_dir = |b| match b {
        b'0' => b'R',
        b'1' => b'D',
        b'2' => b'L',
        b'3' => b'U',
        _ => panic!(),
    };
    let hex_to_num = |b| match b {
        b'a'..=b'f' => (b - b'a' + 10) as usize,
        b'0'..=b'9' => (b - b'0') as usize,
        _ => panic!(),
    };
    solve(input.iter().map(|&(_, _, exa)| {
        let dir = hex_to_dir(exa[5]);
        let len = exa[..5].iter().fold(0, |acc, &b| 16 * acc + hex_to_num(b));
        (dir, len)
    }))
}
