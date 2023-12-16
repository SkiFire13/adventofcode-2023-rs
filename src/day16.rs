#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn solve(input: &Input, start: (i8, i8), v: (i8, i8)) -> usize {
    let mut queue = vec![(start, v)];
    let mut seen = FxHashSet::default();

    while let Some(((x, y), (dx, dy))) = queue.pop() {
        let Some(tile) = input.iget((x as isize, y as isize)) else {
            continue;
        };

        let dir_id = (dx + dy + 1 + (dx > dy) as i8) as usize * input.w() * input.h();
        let idx = dir_id as u16 + input.w() as u16 * y as u16 + x as u16;
        if !seen.insert(idx) {
            continue;
        }

        match (tile, (dx, dy)) {
            (b'.', _) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'|', (0, _)) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'-', (_, 0)) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'|', (_, 0)) => queue.extend([((x, y + 1), (0, 1)), ((x, y - 1), (0, -1))]),
            (b'-', (0, _)) => queue.extend([((x + 1, y), (1, 0)), ((x - 1, y), (-1, 0))]),
            (b'/', _) => queue.push(((x - dy, y - dx), (-dy, -dx))),
            (b'\\', _) => queue.push(((x + dy, y + dx), (dy, dx))),
            _ => panic!(),
        }
    }

    seen.into_iter()
        .map(|idx| idx % (input.h() * input.w()) as u16)
        .unique()
        .count()
}

pub fn part1(input: &Input) -> usize {
    solve(input, (0, 0), (1, 0))
}

pub fn part2(input: &Input) -> usize {
    let (w, h) = (input.w(), input.h());
    let top = (0..w).into_par_iter().map(|x| ((x, 0), (0, 1)));
    let bottom = (0..w).into_par_iter().map(|x| ((x, h - 1), (0, -1)));
    let left = (0..h).into_par_iter().map(|y| ((0, y), (1, 0)));
    let right = (0..h).into_par_iter().map(|y| ((w - 1, y), (-1, 0)));

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|((x, y), (dx, dy))| solve(input, (x as i8, y as i8), (dx as i8, dy as i8)))
        .max()
        .unwrap()
}
