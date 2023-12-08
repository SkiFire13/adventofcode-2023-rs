#[allow(unused_imports)]
use super::prelude::*;

pub enum LR {
    L,
    R,
}

pub struct Input {
    lr: Vec<LR>,
    nodes: Vec<[u8; 3]>,
    left: Vec<usize>,
    right: Vec<usize>,
}

pub fn input_generator(input: &str) -> Input {
    let (lr, paths) = input.split_once("\n\n").unwrap();

    let lr = lr
        .bytes()
        .map(|b| if b == b'L' { LR::L } else { LR::R })
        .collect();

    let (nodes, ((left, right), node_map)) = paths
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let node: [u8; 3] = line[..3].as_bytes().try_into().unwrap();
            let left: [u8; 3] = line[7..10].as_bytes().try_into().unwrap();
            let right: [u8; 3] = line[12..15].as_bytes().try_into().unwrap();
            (node, ((left, right), (node, i)))
        })
        .unzip::<_, _, _, ((Vec<_>, Vec<_>), FxHashMap<_, _>)>();

    let left = left.into_iter().map(|node| node_map[&node]).collect();
    let right = right.into_iter().map(|node| node_map[&node]).collect();

    Input { lr, nodes, left, right }
}

fn solve(input: &Input, start: usize, stop: impl Fn(&[u8; 3]) -> bool) -> usize {
    let mut steps = 0;
    let mut curr = start;
    while !stop(&input.nodes[curr]) {
        for l in &input.lr {
            curr = match l {
                LR::L => input.left[curr],
                LR::R => input.right[curr],
            };
            steps += 1;
        }
    }
    steps
}

pub fn part1(input: &Input) -> usize {
    let aaa = input
        .nodes
        .iter()
        .position(|label| label == b"AAA")
        .unwrap();
    solve(input, aaa, |curr| curr == b"ZZZ")
}

pub fn part2(input: &Input) -> usize {
    input
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, &curr)| curr[2] == b'A')
        .map(|(i, _)| solve(input, i, |curr| curr[2] == b'Z'))
        .fold(1, num::integer::lcm)
}
