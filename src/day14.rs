#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    for y in 1..input.h() {
        for x in 0..input.w() {
            let mut y = y;
            while y > 0 && input[(x, y)] == b'O' && input[(x, y - 1)] == b'.' {
                input[(x, y)] = b'.';
                input[(x, y - 1)] = b'O';
                y -= 1;
            }
        }
    }

    let mut tot = 0;

    for y in 0..input.h() {
        for x in 0..input.w() {
            if input[(x, y)] == b'O' {
                tot += input.h() - y;
            }
        }
    }
    tot
}

pub fn part2(input: &Input) -> usize {
    let mut seen = FxHashMap::default();

    let mut last_acc = vec![0; input.w()];
    let mut input = input.clone();

    let mut curr = 0;
    while curr < 1000000000 {
        seen.insert(input.clone(), curr);

        last_acc.fill(0);
        for y in 0..input.h() {
            for (x, last) in last_acc.iter_mut().enumerate() {
                if input[(x, y)] == b'O' {
                    input[(x, y)] = b'.';
                    input[(x, *last)] = b'O';
                    *last += 1;
                } else if input[(x, y)] == b'#' {
                    *last = y + 1;
                }
            }
        }

        for y in 0..input.h() {
            let mut last = 0;
            for x in 0..input.w() {
                if input[(x, y)] == b'O' {
                    input[(x, y)] = b'.';
                    input[(last, y)] = b'O';
                    last += 1;
                } else if input[(x, y)] == b'#' {
                    last = x + 1;
                }
            }
        }

        last_acc.fill(input.h());
        for y in (0..input.h()).rev() {
            for (x, last) in last_acc.iter_mut().enumerate().rev() {
                if input[(x, y)] == b'O' {
                    *last -= 1;
                    input[(x, y)] = b'.';
                    input[(x, *last)] = b'O';
                } else if input[(x, y)] == b'#' {
                    *last = y;
                }
            }
        }

        for y in (0..input.h()).rev() {
            let mut last = input.w();
            for x in (0..input.w()).rev() {
                if input[(x, y)] == b'O' {
                    last -= 1;
                    input[(x, y)] = b'.';
                    input[(last, y)] = b'O';
                } else if input[(x, y)] == b'#' {
                    last = x;
                }
            }
        }

        curr += 1;

        if let Some(&old) = seen.get(&input) {
            let cycle_len = curr - old;
            let cycle_offset = old;
            let answer = (1000000000 - cycle_offset) % cycle_len + cycle_offset;
            input = seen
                .drain()
                .find(|&(_, i)| i == answer)
                .map(|(grid, _)| grid)
                .unwrap();
            break;
        }
    }

    let mut tot = 0;
    for y in 0..input.h() {
        for x in 0..input.w() {
            if input[(x, y)] == b'O' {
                tot += input.h() - y;
            }
        }
    }
    tot
}
