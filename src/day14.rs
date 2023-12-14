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

    let mut input = input.clone();

    let mut curr = 0;
    while curr < 1000000000 {
        seen.insert(input.clone(), curr);

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

        for x in 1..input.w() {
            for y in 0..input.h() {
                let mut x = x;
                while x > 0 && input[(x, y)] == b'O' && input[(x - 1, y)] == b'.' {
                    input[(x, y)] = b'.';
                    input[(x - 1, y)] = b'O';
                    x -= 1;
                }
            }
        }

        for y in (0..input.h() - 1).rev() {
            for x in 0..input.w() {
                let mut y = y;
                while y < input.h() - 1 && input[(x, y)] == b'O' && input[(x, y + 1)] == b'.' {
                    input[(x, y)] = b'.';
                    input[(x, y + 1)] = b'O';
                    y += 1;
                }
            }
        }

        for x in (0..input.w() - 1).rev() {
            for y in 0..input.h() {
                let mut x = x;
                while x < input.w() - 1 && input[(x, y)] == b'O' && input[(x + 1, y)] == b'.' {
                    input[(x, y)] = b'.';
                    input[(x + 1, y)] = b'O';
                    x += 1;
                }
            }
        }

        curr += 1;

        if let Some(&old) = seen.get(&input) {
            let cycle_len = old - curr;
            let cycle_offset = old;
            let answer = (1000000000 - cycle_offset) % cycle_len + cycle_offset;
            input = seen
                .iter()
                .find(|(_, &i)| i == answer)
                .map(|(grid, _)| grid.clone())
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
