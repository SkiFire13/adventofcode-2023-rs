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

pub fn part1(input: &Input) -> usize {
    let (seeds, maps) = input;
    seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find(|&&(_, start, len)| (start..start + len).contains(&seed))
                    .map(|(dest, start, _)| seed - start + dest)
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> usize {
    let (seeds, maps) = input;

    let mut seeds = seeds
        .iter()
        .tuples()
        .map(|(&start, &len)| (start, start + len))
        .collect::<Vec<(_, _)>>();
    let mut new_seeds = Vec::with_capacity(seeds.len());

    for map in maps {
        let map = map
            .iter()
            .map(|&(dest, start, len)| (start, start + len, dest))
            .sorted()
            .collect::<Vec<_>>();

        for &(mut seed_start, seed_end) in &seeds {
            for &(map_start, map_end, map_dest) in &map {
                if map_end <= seed_start {
                    continue;
                }
                if seed_end <= map_start {
                    break;
                }

                if seed_start < map_start {
                    new_seeds.push((seed_start, map_start));
                    seed_start = map_start;
                }

                let end = min(seed_end, map_end);
                let dest = map_dest + seed_start - map_start;
                new_seeds.push((dest, dest + end - seed_start));
                seed_start = end;
            }

            if seed_start != seed_end {
                new_seeds.push((seed_start, seed_end));
            }
        }

        mem::swap(&mut seeds, &mut new_seeds);
        new_seeds.clear();
        seeds.sort();
    }

    let (start, _) = seeds[0];
    start
}
