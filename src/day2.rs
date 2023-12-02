#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u32, Vec<(u32, u32, u32)>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (game, rest) = line.split_once(": ").unwrap();
            let game = game[5..].parse().unwrap();

            let bags = rest
                .split("; ")
                .map(|bag| {
                    let (mut r, mut g, mut b) = (0, 0, 0);
                    for ball in bag.split(", ") {
                        let (n, color) = ball.split_once(' ').unwrap();
                        let n = n.parse().unwrap();
                        match color {
                            "red" => r = n,
                            "green" => g = n,
                            "blue" => b = n,
                            _ => unreachable!(),
                        }
                    }
                    (r, g, b)
                })
                .collect();
            (game, bags)
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter(|(_, bags)| {
            let rok = bags.iter().all(|&(r, _, _)| r <= 12);
            let gok = bags.iter().all(|&(_, g, _)| g <= 13);
            let bok = bags.iter().all(|&(_, _, b)| b <= 14);
            rok && gok && bok
        })
        .map(|&(id, _)| id)
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|(_, bags)| {
            let maxr = bags.iter().map(|(r, _, _)| r).max().unwrap();
            let maxg = bags.iter().map(|(_, g, _)| g).max().unwrap();
            let maxb = bags.iter().map(|(_, _, b)| b).max().unwrap();
            maxr * maxg * maxb
        })
        .sum()
}
