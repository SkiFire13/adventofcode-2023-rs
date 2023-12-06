#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<u64>, Vec<u64>);

pub fn input_generator(input: &str) -> Input {
    let (times, records) = input.split_once('\n').unwrap();
    let times = times[9..]
        .trim()
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let records = records[9..]
        .trim()
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (times, records)
}

pub fn part1(input: &Input) -> u64 {
    let (times, records) = input;
    let mut acc = 1;
    for (&time, &record) in iter::zip(times, records) {
        let mut wins = 0;
        for j in 0..time {
            let v = j;
            let tr = time - j;
            let dist = v * tr;
            wins += (dist > record) as u64;
        }
        acc *= wins;
    }
    acc
}

pub fn part2(input: &Input) -> u64 {
    let (times, records) = input;

    let concat = |acc, &n| {
        let mut i = 10;
        while i < n {
            i *= 10;
        }
        acc * i + n
    };

    let time = times.iter().fold(0, concat);
    let record = records.iter().fold(0, concat);

    let ds = f64::sqrt((time * time - 4 * record) as f64);
    let min = (time - ds.floor() as u64) / 2;
    let max = (time + ds.ceil() as u64) / 2;

    max - min + 1
}
