#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<LR>, FxHashMap<[u8; 3], ([u8; 3], [u8; 3])>);

pub enum LR {
    L,
    R,
}

pub fn input_generator(input: &str) -> Input {
    let (lr, paths) = input.split_once("\n\n").unwrap();

    let lr = lr
        .bytes()
        .map(|b| if b == b'l' { LR::L } else { LR::R })
        .collect();

    let paths = paths
        .lines()
        .map(|line| {
            let curr = line[..3].as_bytes().try_into().unwrap();
            let left = line[7..10].as_bytes().try_into().unwrap();
            let right = line[12..15].as_bytes().try_into().unwrap();
            (curr, (left, right))
        })
        .collect();
    (lr, paths)
}

fn solve(input: &Input, start: [u8; 3], stop: impl Fn(&[u8; 3]) -> bool) -> usize {
    let (lr, paths) = input;
    let mut steps = 0;
    let mut curr = start;
    loop {
        if stop(&curr) {
            return steps;
        }
        for l in lr {
            curr = match l {
                LR::L => paths[&curr].0,
                LR::R => paths[&curr].1,
            };
            steps += 1;
        }
    }
}

pub fn part1(input: &Input) -> usize {
    solve(input, *b"AAA", |curr| curr == b"ZZZ")
}

pub fn part2(input: &Input) -> usize {
    let (_, paths) = input;
    paths
        .keys()
        .copied()
        .filter(|&curr| curr[2] == b'A')
        .map(|curr| solve(input, curr, |curr| curr[2] == b'Z'))
        .fold(1, num::integer::lcm)
}
