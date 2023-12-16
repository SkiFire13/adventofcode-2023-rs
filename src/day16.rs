#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn solve(input: &Input, start: (isize, isize), v: (isize, isize)) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = FxHashSet::default();

    let (nx, ny) = start;
    let (dx, dy) = v;

    match (input[(nx, ny)], (dx, dy)) {
        (b'.', _) => queue.push_back(((nx, ny), (dx, dy))),
        (b'|', (0, _)) => queue.push_back(((nx, ny), (dx, dy))),
        (b'-', (_, 0)) => queue.push_back(((nx, ny), (dx, dy))),
        (b'|', (_, 0)) => queue.extend([((nx, ny), (0, 1)), ((nx, ny), (0, -1))]),
        (b'-', (0, _)) => queue.extend([((nx, ny), (1, 0)), ((nx, ny), (-1, 0))]),
        (b'/', _) => queue.push_back(((nx, ny), (-dy, -dx))),
        (b'\\', _) => queue.push_back(((nx, ny), (dy, dx))),
        _ => panic!(),
    }

    while let Some(((x, y), (dx, dy))) = queue.pop_front() {
        if !seen.insert(((x, y), (dx, dy))) {
            continue;
        }

        let (nx, ny) = (x + dx, y + dy);

        if nx < 0 || nx >= input.w() as isize || ny < 0 || ny >= input.h() as isize {
            continue;
        }

        match (input[(nx, ny)], (dx, dy)) {
            (b'.', _) => queue.push_back(((nx, ny), (dx, dy))),
            (b'|', (0, _)) => queue.push_back(((nx, ny), (dx, dy))),
            (b'-', (_, 0)) => queue.push_back(((nx, ny), (dx, dy))),
            (b'|', (_, 0)) => queue.extend([((nx, ny), (0, 1)), ((nx, ny), (0, -1))]),
            (b'-', (0, _)) => queue.extend([((nx, ny), (1, 0)), ((nx, ny), (-1, 0))]),
            (b'/', _) => queue.push_back(((nx, ny), (-dy, -dx))),
            (b'\\', _) => queue.push_back(((nx, ny), (dy, dx))),
            _ => panic!(),
        }
    }

    seen.into_iter().map(|(pos, _)| pos).unique().count()
}

pub fn part1(input: &Input) -> usize {
    solve(input, (0, 0), (1, 0))
}

pub fn part2(input: &Input) -> usize {
    let top = (0..input.w() as isize)
        .into_par_iter()
        .map(|x| ((x, 0), (0, 1)));
    let bottom = (0..input.w() as isize)
        .into_par_iter()
        .map(|x| ((x, input.h() as isize - 1), (0, -1)));
    let left = (0..input.h() as isize)
        .into_par_iter()
        .map(|y| ((0, y), (1, 0)));
    let right = (0..input.h() as isize)
        .into_par_iter()
        .map(|y| ((input.w() as isize - 1, y), (-1, 0)));

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|(pos, v)| solve(input, pos, v))
        .max()
        .unwrap()
}
