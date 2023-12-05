#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<usize>, [Vec<(usize, usize, usize)>; 7]);

pub fn input_generator(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let seeds = input.next().unwrap();
    let seeds = seeds.split_whitespace().skip(1);
    let seeds = seeds.map(|n| n.parse::<usize>().unwrap()).collect();

    let maps = array::from_fn(|_| {
        let input = input.next().unwrap();
        let lines = input.lines().skip(1);
        lines
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    });

    (seeds, maps)
}

fn map(seed: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for &(dest, start, len) in map {
        if (start..start + len).contains(&seed) {
            return seed - start + dest;
        }
    }
    seed
}

pub fn part1(input: &Input) -> usize {
    let (seeds, maps) = input;
    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, map))
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> usize {
    let (seeds, maps) = input;
    seeds
        .iter()
        .tuples()
        .map(|(&seed_start, &seed_len)| {
            (seed_start..seed_start + seed_len)
                .into_par_iter()
                .map(|seed| maps.iter().fold(seed, map))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}
