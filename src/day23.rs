#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn simplify_graph<const PART2: bool>(input: &Input) -> Vec<ArrayVec<(usize, usize), 4>> {
    let start = (0..input.w()).find(|&x| input[(x, 0)] == b'.');
    let end = (0..input.w()).find(|&x| input[(x, input.h() - 1)] == b'.');
    let (start, end) = (start.unwrap(), end.unwrap());

    let mut node_poss = vec![(start, 0), (end, input.h() - 1)];
    let mut node_edges = vec![ArrayVec::new(), ArrayVec::new()];
    let mut pos_to_node = FxHashMap::from_iter([(node_poss[0], 0), (node_poss[1], 1)]);
    let mut explored = FxHashSet::default();
    let mut to_visit = vec![0];

    explored.reserve(128);
    pos_to_node.reserve(64);

    while let Some(node) = to_visit.pop() {
        for (mut x, mut y) in input.plus_neighbours(node_poss[node]) {
            if input[(x, y)] == b'#' || !explored.insert((x, y)) {
                continue;
            }

            let mut d = 1;
            let (mut px, mut py) = node_poss[node];
            let (mut forward, mut backward) = (true, true);

            loop {
                let (dx, dy) = (x as isize - px as isize, y as isize - py as isize);
                match (if PART2 { b'.' } else { input[(x, y)] }, dx, dy) {
                    (b'<', 1, 0) | (b'>', -1, 0) | (b'^', 0, 1) | (b'v', 0, -1) => forward = false,
                    (b'<', -1, 0) | (b'>', 1, 0) | (b'^', 0, -1) | (b'v', 0, 1) => backward = false,
                    _ => {}
                }

                if (x, y) == (end, input.h() - 1) {
                    node_edges[node].push((1, d));
                    break;
                }

                let mut iter = input
                    .plus_neighbours((x, y))
                    .filter(|&(nx, ny)| input[(nx, ny)] != b'#')
                    .filter(|&(nx, ny)| (nx, ny) != (px, py));

                let Some((nx, ny)) = iter.next() else { break };

                if iter.next().is_some() {
                    let end_node = *pos_to_node.entry((x, y)).or_insert_with(|| {
                        let new_node = node_poss.len();
                        node_poss.push((x, y));
                        node_edges.push(ArrayVec::new());
                        to_visit.push(new_node);
                        new_node
                    });
                    explored.insert((px, py));
                    forward.then(|| node_edges[node].push((end_node, d)));
                    backward.then(|| node_edges[end_node].push((node, d)));

                    break;
                }

                (px, py, x, y, d) = (x, y, nx, ny, d + 1);
            }
        }
    }

    node_edges
}

pub fn part1(input: &Input) -> usize {
    fn solve(edges: &[ArrayVec<(usize, usize), 4>], seen: u64, node: usize, d: usize) -> usize {
        edges[node]
            .iter()
            .filter(|&&(next, _)| seen & (1 << next) == 0)
            .map(|&(next, cost)| solve(edges, seen | (1 << next), next, d + cost))
            .max()
            .unwrap_or(if node == 1 { d } else { 0 })
    }
    solve(&simplify_graph::<false>(input), 0, 0, 0)
}

pub fn part2(input: &Input) -> usize {
    fn solve(edges: &[ArrayVec<(usize, usize), 4>], seen: u64, node: usize, d: usize) -> usize {
        let branches = edges[node]
            .iter()
            .filter(|&&(next, _)| seen & (1 << next) == 0)
            .map(|&(next, cost)| move || solve(edges, seen | (1 << next), next, d + cost))
            .collect::<ArrayVec<_, 4>>();

        match &branches[..] {
            &[] if node == 1 => d,
            &[] => 0,
            &[f] => f(),
            &[f1, f2] => {
                let (d1, d2) = rayon::join(f1, f2);
                max(d1, d2)
            }
            &[f1, f2, f3] => {
                let (d1, (d2, d3)) = rayon::join(f1, || rayon::join(f2, f3));
                max(d1, max(d2, d3))
            }
            _ => unreachable!(),
        }
    }
    solve(&simplify_graph::<true>(input), 0, 0, 0)
}
