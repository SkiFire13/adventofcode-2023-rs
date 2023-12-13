#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Grid<u8>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|grid| Grid::from_input_chars(grid, |c, _, _| c as u8))
        .collect()
}

fn exactly_n(mut iter: impl Iterator, n: usize) -> bool {
    iter.try_fold(n, |acc, _| acc.checked_sub(1)) == Some(0)
}

fn solve(input: &Input, target: usize) -> usize {
    input
        .iter()
        .map(|grid| {
            for row in 1..grid.h() {
                let size = min(row, grid.h() - row);
                let up = &grid.vec[(row - size) * grid.w()..row * grid.w()];
                let down = &grid.vec[row * grid.w()..(row + size) * grid.w()];
                let iter = iter::zip(up.chunks_exact(grid.w()), down.chunks_exact(grid.w()).rev())
                    .flat_map(|(up, down)| iter::zip(up, down))
                    .filter(|(up, down)| up != down);
                if exactly_n(iter, target) {
                    return 100 * row;
                }
            }

            for col in 1..grid.w() {
                let size = min(col, grid.w() - col);
                let rows = grid.vec.chunks_exact(grid.w());
                let iter = rows
                    .flat_map(|row| {
                        let left = &row[col - size..col];
                        let right = row[col..][..size].iter().rev();
                        iter::zip(left, right)
                    })
                    .filter(|(up, down)| up != down);
                if exactly_n(iter, target) {
                    return col;
                }
            }

            unreachable!()
        })
        .sum()
}

pub fn part1(input: &Input) -> usize {
    solve(input, 0)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 1)
}
