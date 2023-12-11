#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<bool>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c == '#')
}

fn solve(input: &Input, factor: usize) -> usize {
    let empty_cols = (0..input.w())
        .filter(|&x| (0..input.h()).all(|y| input[(x, y)] == false))
        .collect::<Vec<_>>();
    let empty_rows = (0..input.h())
        .filter(|&y| (0..input.w()).all(|x| input[(x, y)] == false))
        .collect::<Vec<_>>();

    let nodes = input
        .iter()
        .filter(|&(_, &b)| b)
        .map(|((x, y), _)| {
            let x = x + empty_cols.iter().take_while(|&&rx| rx < x).count() * factor;
            let y = y + empty_rows.iter().take_while(|&&ry| ry < y).count() * factor;
            (x, y)
        })
        .collect::<Vec<_>>();

    nodes
        .iter()
        .enumerate()
        .flat_map(|(i, n)| nodes[i + 1..].iter().map(move |m| (n, m)))
        .map(|(&(ix, iy), &(jx, jy))| ix.abs_diff(jx) + iy.abs_diff(jy))
        .sum()
}

pub fn part1(input: &Input) -> usize {
    solve(input, 1)
}

pub fn part2(input: &Input) -> usize {
    solve(input, 1_000_000 - 1)
}
