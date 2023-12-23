#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn solve(
    input: &Input,
    seen: &mut FxHashSet<(usize, usize)>,
    (x, y): (usize, usize),
    d: usize,
) -> usize {
    if y == input.h() - 1 {
        return d;
    }

    let mut best = 0;

    for (nx, ny) in input.plus_neighbours((x, y)) {
        if input[(nx, ny)] != b'#' {
            let dx = (nx as isize) - (x as isize);
            let dy = (ny as isize) - (y as isize);
            match (input[(nx, ny)], (dx, dy)) {
                (b'<', (1, 0)) => continue,
                (b'>', (-1, 0)) => continue,
                (b'v', (0, -1)) => continue,
                (b'^', (0, 1)) => continue,
                _ => {
                    if seen.insert((nx, ny)) {
                        let nd = solve(input, seen, (nx, ny), d + 1);
                        best = max(best, nd);
                        seen.remove(&(nx, ny));
                    }
                }
            }
        }
    }

    best
}

pub fn part1(input: &Input) -> usize {
    let mut seen = FxHashSet::default();
    let start = (0..input.w()).find(|&x| input[(x, 0)] == b'.').unwrap();
    solve(input, &mut seen, (start, 0), 0)
}

fn solve2(
    input: &Input,
    seen: &mut FxHashSet<(usize, usize)>,
    (x, y): (usize, usize),
    d: usize,
) -> usize {
    if y == input.h() - 1 {
        return d;
    }

    let mut best = 0;

    for (nx, ny) in input.plus_neighbours((x, y)) {
        if input[(nx, ny)] != b'#' {
            if seen.insert((nx, ny)) {
                stacker::maybe_grow(32 * 1024, 1024 * 1024 * 10, || {
                    let nd = solve2(input, seen, (nx, ny), d + 1);
                    best = max(best, nd);
                    seen.remove(&(nx, ny));
                });
            }
        }
    }

    best
}

pub fn part2(input: &Input) -> usize {
    let mut seen = FxHashSet::default();
    let start = (0..input.w()).find(|&x| input[(x, 0)] == b'.').unwrap();
    solve2(input, &mut seen, (start, 0), 0)
}
