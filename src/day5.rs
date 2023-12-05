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

fn map_range(
    mut seed_start: usize,
    mut seed_len: usize,
    mut map: &[(usize, usize, usize)],
    mut f: impl FnMut(usize, usize),
) {
    while seed_len != 0 {
        let Some(idx) = map
            .iter()
            .position(|&(_, map_start, map_len)| map_start + map_len > seed_start)
            .filter(|&idx| map[idx].1 < seed_start + seed_len)
        else {
            f(seed_start, seed_len);
            return;
        };

        let (mut map_dest, mut map_start, mut map_len) = map[idx];
        map = &map[idx + 1..];

        if map_start < seed_start {
            map_start = seed_start;
            map_dest += seed_start - map_start;
            map_len -= seed_start - map_start;
        } else if map_start > seed_start {
            let len = map_start - seed_start;
            f(seed_start, len);
            seed_start += len;
            seed_len -= len;
        }

        let len = min(map_len, seed_len);
        f(map_dest, map_dest + len);
        seed_start += len;
        seed_len -= len;
    }
}

pub fn part2(input: &Input) -> usize {
    let (seeds, maps) = input;

    let mut maps = maps.clone();
    maps.iter_mut()
        .for_each(|map| map.sort_by_key(|&(dest, start, len)| (start, len, dest)));

    let mut min_seed = usize::MAX;
    let mut map_min = |start, _| min_seed = min(min_seed, start);
    let mut map6 = |start, len| map_range(start, len, &maps[6], &mut map_min);
    let mut map5 = |start, len| map_range(start, len, &maps[5], &mut map6);
    let mut map4 = |start, len| map_range(start, len, &maps[4], &mut map5);
    let mut map3 = |start, len| map_range(start, len, &maps[3], &mut map4);
    let mut map2 = |start, len| map_range(start, len, &maps[2], &mut map3);
    let mut map1 = |start, len| map_range(start, len, &maps[1], &mut map2);
    let mut map0 = |start, len| map_range(start, len, &maps[0], &mut map1);
    for (&start, &len) in seeds.iter().tuples() {
        map0(start, len)
    }
    min_seed
}
