#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input {
    input.trim().as_bytes().split(|&b| b == b',').collect()
}

fn hash(s: &[u8]) -> u8 {
    s.iter()
        .fold(0, |acc, &b| acc.wrapping_add(b).wrapping_mul(17))
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|s| hash(s) as u32).sum()
}

pub fn part2(input: &Input) -> usize {
    let mut lenses = vec![Vec::new(); 256];

    for instr in input {
        let label = instr.split(|&b| b == b'-' || b == b'=').next().unwrap();
        let lens = &mut lenses[hash(label) as usize];
        match instr[label.len()] {
            b'-' => {
                if let Some(idx) = lens.iter().position(|&(s, _)| s == label) {
                    lens.remove(idx);
                }
            }
            b'=' => {
                let n = instr[label.len() + 1..]
                    .iter()
                    .fold(0, |acc, &b| 10 * acc + (b - b'0') as u32);
                if let Some(idx) = lens.iter().position(|&(s, _)| s == label) {
                    lens[idx].1 = n;
                } else {
                    lens.push((label, n));
                }
            }
            _ => unreachable!(),
        }
    }

    lenses
        .iter()
        .enumerate()
        .flat_map(|(l, lens)| lens.iter().enumerate().map(move |a| (l, a)))
        .map(|(l, (lidx, &(_, n)))| (l + 1) * (lidx + 1) * n as usize)
        .sum()
}
