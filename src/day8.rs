#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<bool>, FxHashMap<[u8; 3], ([u8; 3], [u8; 3])>);

pub fn input_generator(input: &str) -> Input {
    let (lr, paths) = input.split_once("\n\n").unwrap();

    let lr = lr.bytes().map(|b| b == b'l').collect();

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

pub fn part1(input: &Input) -> usize {
    let (lr, paths) = input;
    let mut curr = *b"AAA";
    let mut steps = 0;
    loop {
        if curr == *b"ZZZ" {
            return steps;
        }
        for &l in lr {
            if l {
                curr = paths[&curr].0;
            } else {
                curr = paths[&curr].1;
            }
            steps += 1;
        }
    }
}

pub fn part2(input: &Input) -> usize {
    let (lr, paths) = input;

    paths
        .keys()
        .copied()
        .filter(|&path| path[2] == b'A')
        .map(|path| {
            let mut curr = path;
            let mut steps = 0;
            loop {
                if curr[2] == b'Z' {
                    return steps;
                }
                for &l in lr {
                    if l {
                        curr = paths[&curr].0;
                    } else {
                        curr = paths[&curr].1;
                    }
                    steps += 1;
                }
            }
        })
        .fold(1, |acc, steps| {
            let (_, lcm) = gcd_lcm(acc, steps);
            lcm
        })
}
