#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Grid<u8>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|grid| Grid::from_input_chars(grid, |c, _, _| c as u8))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|grid| {
            for row in 1..grid.h() {
                let up = &grid.vec[row.saturating_sub(grid.h() - row) * grid.w()..row * grid.w()];
                let down = &grid.vec[row * grid.w()..min(2 * row, grid.h()) * grid.w()];

                if iter::zip(up.chunks_exact(grid.w()), down.chunks_exact(grid.w()).rev())
                    .all(|(up, down)| up == down)
                {
                    return 100 * row;
                }
            }

            let grid = Grid::with_dimensions_init(grid.h(), grid.w(), |x, y| grid[(y, x)]);
            for row in 1..grid.h() {
                let up = &grid.vec[row.saturating_sub(grid.h() - row) * grid.w()..row * grid.w()];
                let down = &grid.vec[row * grid.w()..min(2 * row, grid.h()) * grid.w()];

                if iter::zip(up.chunks_exact(grid.w()), down.chunks_exact(grid.w()).rev())
                    .all(|(up, down)| up == down)
                {
                    return row;
                }
            }

            unreachable!()
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|grid| {
            for row in 1..grid.h() {
                let up = &grid.vec[row.saturating_sub(grid.h() - row) * grid.w()..row * grid.w()];
                let down = &grid.vec[row * grid.w()..min(2 * row, grid.h()) * grid.w()];

                if iter::zip(up.chunks_exact(grid.w()), down.chunks_exact(grid.w()).rev())
                    .flat_map(|(r1, r2)| iter::zip(r1, r2))
                    .filter(|&(up, down)| up != down)
                    .exactly_one()
                    .is_ok()
                {
                    return 100 * row;
                }
            }

            let grid = Grid::with_dimensions_init(grid.h(), grid.w(), |x, y| grid[(y, x)]);
            for row in 1..grid.h() {
                let up = &grid.vec[row.saturating_sub(grid.h() - row) * grid.w()..row * grid.w()];
                let down = &grid.vec[row * grid.w()..min(2 * row, grid.h()) * grid.w()];
                if iter::zip(up.chunks_exact(grid.w()), down.chunks_exact(grid.w()).rev())
                    .flat_map(|(r1, r2)| iter::zip(r1, r2))
                    .filter(|&(up, down)| up != down)
                    .exactly_one()
                    .is_ok()
                {
                    return row;
                }
            }

            unreachable!()
        })
        .sum()
}
