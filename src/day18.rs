use core::panic;

#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u8, usize, [u8; 6])>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let dir = line.as_bytes()[0];
            let (len, color) = line[2..].split_once(' ').unwrap();
            let len = len.parse().unwrap();
            let color = color[2..8].as_bytes().try_into().unwrap();
            (dir, len, color)
        })
        .collect()
}

pub fn part1(input: &Input) -> isize {
    let (mut minx, mut maxx) = (isize::MAX, isize::MIN);
    let (mut miny, mut maxy) = (isize::MAX, isize::MIN);
    let (mut x, mut y) = (0, 0);
    for &(dir, len, _) in input {
        let (dx, dy) = match dir {
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'D' => (0, -1),
            b'U' => (0, 1),
            _ => panic!(),
        };
        (x, y) = (x + dx * len as isize, y + dy * len as isize);
        (minx, maxx) = (min(x, minx), max(x, maxx));
        (miny, maxy) = (min(y, miny), max(y, maxy));
    }

    let mut cside = b'U';
    let (mut cx, mut cy) = (0, -miny + 1);
    let mut area = 0;

    let first = (input[input.len() - 1], input[0], input[1]);
    let last = (input[input.len() - 2], input[input.len() - 1], input[0]);

    let iter = itertools::chain!([first], input.iter().copied().tuple_windows(), [last]);

    for ((dir_p, _, _), (dir, len, _), (dir_n, _, _)) in iter {
        let malus_in = (cside != dir_p) as usize;
        let malus_out = (cside == dir_n) as usize;
        let clen = len + 1 - malus_in - malus_out;

        dbg!(clen);
        // assert!(cy >= 0);

        match dir {
            b'R' => area += clen as isize * cy,
            b'L' => area -= clen as isize * cy,
            _ => {}
        }

        let (dx, dy) = match dir {
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'D' => (0, 1),
            b'U' => (0, -1),
            _ => panic!(),
        };
        (cx, cy) = (cx + dx * clen as isize, cy + dy * clen as isize);
        cside = match (cside, dir, dir_n) {
            (b'U', b'R', b'U') | (b'U', b'L', b'D') => b'L',
            (b'U', b'L', b'U') | (b'U', b'R', b'D') => b'R',
            (b'D', b'R', b'U') | (b'D', b'L', b'D') => b'R',
            (b'D', b'L', b'U') | (b'D', b'R', b'D') => b'L',

            (b'L', b'U', b'R') | (b'L', b'D', b'L') => b'U',
            (b'L', b'D', b'R') | (b'L', b'U', b'L') => b'D',
            (b'R', b'U', b'R') | (b'R', b'D', b'L') => b'D',
            (b'R', b'D', b'R') | (b'R', b'U', b'L') => b'U',
            _ => panic!(),
        }
    }

    area.abs()
}

pub fn part2(input: &Input) -> isize {
    let input = input
        .iter()
        .map(|&(_, _, exa)| {
            let dir = match exa[5] {
                b'0' => b'R',
                b'1' => b'D',
                b'2' => b'L',
                b'3' => b'U',
                _ => panic!(),
            };
            let len = exa[..5].iter().fold(0, |acc, &b| {
                16 * acc
                    + match b {
                        b'a'..=b'f' => (b - b'a' + 10) as usize,
                        b'0'..=b'9' => (b - b'0') as usize,
                        _ => panic!(),
                    }
            });
            (dir, len)
        })
        .collect::<Vec<_>>();

    let (mut minx, mut maxx) = (isize::MAX, isize::MIN);
    let (mut miny, mut maxy) = (isize::MAX, isize::MIN);
    let (mut x, mut y) = (0, 0);
    for &(dir, len) in &input {
        let (dx, dy) = match dir {
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'D' => (0, -1),
            b'U' => (0, 1),
            _ => panic!(),
        };
        (x, y) = (x + dx * len as isize, y + dy * len as isize);
        (minx, maxx) = (min(x, minx), max(x, maxx));
        (miny, maxy) = (min(y, miny), max(y, maxy));
    }

    // TODO: Depends on input + inside/outside
    let mut cside = b'D';
    let (mut cx, mut cy) = (0, 0);
    let mut area = 0;

    let first = (input[input.len() - 1], input[0], input[1]);
    let last = (input[input.len() - 2], input[input.len() - 1], input[0]);

    let iter = itertools::chain!([first], input.iter().copied().tuple_windows(), [last]);

    for ((dir_p, _), (dir, len), (dir_n, _)) in iter {
        let malus_in = (cside != dir_p) as usize;
        let malus_out = (cside == dir_n) as usize;
        let clen = len + 1 - malus_in - malus_out;

        dbg!(clen);
        // assert!(cy >= 0);

        match dir {
            b'R' => area += clen as isize * cy,
            b'L' => area -= clen as isize * cy,
            _ => {}
        }

        let (dx, dy) = match dir {
            b'L' => (-1, 0),
            b'R' => (1, 0),
            b'D' => (0, 1),
            b'U' => (0, -1),
            _ => panic!(),
        };
        (cx, cy) = (cx + dx * clen as isize, cy + dy * clen as isize);
        cside = match (cside, dir, dir_n) {
            (b'U', b'R', b'U') | (b'U', b'L', b'D') => b'L',
            (b'U', b'L', b'U') | (b'U', b'R', b'D') => b'R',
            (b'D', b'R', b'U') | (b'D', b'L', b'D') => b'R',
            (b'D', b'L', b'U') | (b'D', b'R', b'D') => b'L',

            (b'L', b'U', b'R') | (b'L', b'D', b'L') => b'U',
            (b'L', b'D', b'R') | (b'L', b'U', b'L') => b'D',
            (b'R', b'U', b'R') | (b'R', b'D', b'L') => b'D',
            (b'R', b'D', b'R') | (b'R', b'U', b'L') => b'U',
            _ => panic!(),
        }
    }

    area.abs()
}
