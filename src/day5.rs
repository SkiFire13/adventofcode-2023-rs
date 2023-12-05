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

    let mut seeds = seeds.iter().copied().tuples().collect::<Vec<(_, _)>>();
    let mut new_seeds = Vec::with_capacity(seeds.len());

    for map in maps {
        let map = map
            .iter()
            .copied()
            .sorted_by_key(|&(dest, start, len)| (start, len, dest))
            .collect::<Vec<_>>();

        for &(mut seed_start, mut seed_len) in &seeds {
            while seed_len != 0 {
                let idx = map
                    .partition_point(|&(_, map_start, map_len)| map_start + map_len <= seed_start);
                if idx >= map.len() {
                    new_seeds.push((seed_start, seed_len));
                    break;
                }

                let (mut map_dest, map_start, mut map_len) = map[idx];
                if map_start >= seed_start + seed_len {
                    new_seeds.push((seed_start, seed_len));
                    break;
                }

                if map_start < seed_start {
                    let skip = seed_start - map_start;
                    map_dest += skip;
                    map_len -= skip;
                } else if map_start > seed_start {
                    let len = map_start - seed_start;
                    new_seeds.push((seed_start, len));
                    seed_start = map_start;
                    seed_len -= len;
                }

                let len = min(map_len, seed_len);
                new_seeds.push((map_dest, len));
                seed_start += len;
                seed_len -= len;
            }
        }

        seeds.clear();
        mem::swap(&mut seeds, &mut new_seeds);

        seeds.sort();

        let mut i = 0;
        for j in 1..seeds.len() {
            let (i_start, i_len) = seeds[i];
            let (j_start, j_len) = seeds[j];
            if i_start + i_len >= j_start {
                let end = max(i_start + i_len, j_start + j_len);
                seeds[i] = (i_start, end - i_start);
            } else {
                if i_len != 0 {
                    i += 1;
                }
                seeds[i] = seeds[j];
            }
        }
        seeds.truncate(i + 1);
    }

    let (start, _) = seeds[0];
    start
}
