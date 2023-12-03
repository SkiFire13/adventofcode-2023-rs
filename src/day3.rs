#[allow(unused_imports)]
use super::prelude::*;
type Input = (
    Vec<usize>,
    FxHashMap<(usize, usize), usize>,
    Vec<(usize, usize, char)>,
);

pub fn input_generator(input: &str) -> Input {
    let mut nums = Vec::new();
    let mut pos_to_nums = FxHashMap::default();
    let mut symbols = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut prev_number = false;
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if !prev_number {
                    nums.push(
                        line[x..]
                            .split(|c: char| !c.is_ascii_digit())
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }

                let num_id = nums.len() - 1;
                pos_to_nums.insert((x, y), num_id);

                prev_number = true;
            } else {
                prev_number = false;

                if c != '.' {
                    symbols.push((x, y, c));
                }
            }
        }
    }

    (nums, pos_to_nums, symbols)
}

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|&(x, y)| x != 0 || y != 0)
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
}

pub fn part1(input: &Input) -> usize {
    let (nums, pos_to_nums, symbols) = input;

    let mut seen = HashSet::new();
    let mut sum = 0;

    for &(x, y, _) in symbols {
        for &n in neighbours(x, y).filter_map(|pos| pos_to_nums.get(&pos)) {
            if seen.insert(n) {
                sum += nums[n];
            }
        }
    }

    sum
}

pub fn part2(input: &Input) -> usize {
    let (nums, pos_to_nums, symbols) = input;

    let mut seen = HashSet::new();
    let mut sum = 0;

    for &(x, y, _) in symbols.iter().filter(|&&(_, _, c)| c == '*') {
        for &n in neighbours(x, y).filter_map(|pos| pos_to_nums.get(&pos)) {
            seen.insert(n);
        }

        if seen.len() >= 2 {
            sum += seen.iter().map(|&idx| nums[idx]).product::<usize>();
        }

        seen.clear();
    }

    sum
}
