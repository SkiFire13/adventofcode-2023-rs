#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Vec<(u16, u16)>>, usize);

pub fn input_generator(input: &str) -> Input {
    let mut node_to_id = FxHashMap::default();
    let mut node_to_edges = Vec::new();
    let mut next_edge_id = 0;

    for line in input.lines() {
        let (key, rest) = line.split_once(": ").unwrap();

        let key_id = *node_to_id.entry(key).or_insert_with(|| {
            node_to_edges.push(Vec::new());
            node_to_edges.len() as u16 - 1
        });

        for other in rest.split(' ') {
            let other_id = *node_to_id.entry(other).or_insert_with(|| {
                node_to_edges.push(Vec::new());
                node_to_edges.len() as u16 - 1
            });

            node_to_edges[key_id as usize].push((other_id, next_edge_id as u16));
            node_to_edges[other_id as usize].push((key_id, next_edge_id as u16 + 1));
            next_edge_id += 2;
        }
    }
    (node_to_edges, next_edge_id)
}

pub fn part1(input: &Input) -> usize {
    let &(ref node_to_edges, edge_count) = input;

    let mut queue = VecDeque::new();
    let mut prev = vec![(0, 0); node_to_edges.len()];
    let mut free_edges = bitbox![1; edge_count * 2];
    let mut seen = bitbox![0; node_to_edges.len()];

    queue.push_back(0);
    seen.set(0, true);
    let mut best_candidate = 0;
    while let Some(curr) = queue.pop_front() {
        best_candidate = curr;
        for &(next, _) in &node_to_edges[curr as usize] {
            if !seen.replace(next as usize, true) {
                queue.push_back(next);
            }
        }
    }

    let random_candidates = [()]
        .into_iter()
        .flat_map(|_| (1..node_to_edges.len() as u16).collect::<HashSet<_>>());
    for end in ichain!([best_candidate], random_candidates) {
        free_edges.fill(true);

        for _ in 0..3 {
            prev.fill((u16::MAX, u16::MAX));
            prev[0] = (u16::MAX - 1, u16::MAX - 1);
            queue.resize(1, 0);

            while let Some(curr) = queue.pop_front() {
                for &(next, edge) in &node_to_edges[curr as usize] {
                    if prev[next as usize] == (u16::MAX, u16::MAX) && free_edges[edge as usize] {
                        prev[next as usize] = (curr, edge);
                        queue.push_back(next);
                        if next == end {
                            break;
                        }
                    }
                }
            }

            let (mut curr, mut edge) = prev[end as usize];
            while (curr, edge) != (u16::MAX - 1, u16::MAX - 1) {
                let inv_edge_free = free_edges[edge as usize ^ 1];
                free_edges.set(edge as usize, !inv_edge_free);
                free_edges.set(edge as usize ^ 1, true);
                (curr, edge) = prev[curr as usize];
            }
        }

        seen.fill(false);
        seen.set(0, true);
        queue.resize(1, 0);
        let mut nseen = 1;
        while let Some(curr) = queue.pop_front() {
            for &(next, edge) in &node_to_edges[curr as usize] {
                if free_edges[edge as usize] && !seen.replace(next as usize, true) {
                    queue.push_back(next);
                    nseen += 1;
                }
            }
        }
        if nseen != node_to_edges.len() {
            return nseen * (node_to_edges.len() - nseen);
        }
    }

    unreachable!()
}
