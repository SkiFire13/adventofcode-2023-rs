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

pub fn part1(input: &Input) -> usize {
    let (times, records) = input;

    iter::zip(times, records)
        .map(|(&time, &record)| (0..time).filter(|v| v * (time - v) > record).count())
        .product()
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
    let mut min = (time - ds.ceil() as u64) / 2;
    let mut max = (time + ds.ceil() as u64) / 2;

    min += (min * (time - min) <= record) as u64;
    max -= (max * (time - max) <= record) as u64;

    max - min + 1
}
