#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

fn solve<E, T, S>(input: &Input, end_cond: E, turn_cond: T, step_cond: S) -> usize
where
    E: Fn(usize) -> bool,
    T: Fn(usize) -> bool,
    S: Fn(usize) -> bool,
{
    let mut seen = FxHashSet::default();
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0, 0, 0, 0)]);

    while let Some((Reverse(loss), x, y, s, dx, dy)) = queue.pop() {
        if x == input.w() as isize - 1 && y == input.h() as isize - 1 && end_cond(s) {
            return loss;
        }

        if !seen.insert((x, y, s, dx, dy)) {
            continue;
        }

        queue.extend(
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter(|&(ndx, ndy)| turn_cond(s) || (ndx, ndy) == (dx, dy))
                .filter(|&(ndx, ndy)| step_cond(s) || (ndx, ndy) != (dx, dy))
                .filter(|&(ndx, ndy)| (ndx, ndy) != (-dx, -dy))
                .map(|(ndx, ndy)| (x + ndx, y + ndy, ndx, ndy))
                .filter(|&(x, y, _, _)| {
                    x >= 0 && y >= 0 && x < input.w() as isize && y < input.h() as isize
                })
                .map(|(x, y, ndx, ndy)| {
                    let s = if (ndx, ndy) == (dx, dy) { s + 1 } else { 1 };
                    let loss = loss + input[(x, y)] as usize;
                    (Reverse(loss), x, y, s, ndx, ndy)
                }),
        );
    }

    unreachable!()
}

pub fn part1(input: &Input) -> usize {
    solve(input, |_| true, |_| true, |s| s < 3)
}

pub fn part2(input: &Input) -> usize {
    solve(input, |s| s >= 4, |s| s == 0 || s >= 4, |s| s < 10)
}
