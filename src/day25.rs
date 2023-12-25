#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Vec<(u16, u16)>>, usize);

pub fn input_generator(input: &str) -> Input {
    let mut node_to_id = FxHashMap::default();
    let mut edges = Vec::new();
    let mut next_edge_id = 0;

    for line in input.lines() {
        let (key, rest) = line.split_once(": ").unwrap();

        let key_id = *node_to_id.entry(key).or_insert_with(|| {
            edges.push(Vec::new());
            edges.len() as u16 - 1
        });

        for other in rest.split(' ') {
            let other_id = *node_to_id.entry(other).or_insert_with(|| {
                edges.push(Vec::new());
                edges.len() as u16 - 1
            });

            edges[key_id as usize].push((other_id, next_edge_id as u16));
            edges[other_id as usize].push((key_id, next_edge_id as u16));
            next_edge_id += 1;
        }
    }
    (edges, next_edge_id)
}

pub fn part1(input: &Input) -> usize {
    let &(ref node_to_edges, edge_count) = input;

    let mut set1 = bitbox![0; node_to_edges.len()];
    let mut min_cut = bitbox![0; edge_count];
    let mut min_cut_len = 0;
    let mut to_visit = Vec::<(u16, u16)>::new();
    let mut visited = bitbox![0; node_to_edges.len()];

    set1.set(0, true);
    visited.set(0, true);
    to_visit.extend(&node_to_edges[0]);

    let mut edges_seen = bitbox![0; edge_count];
    let mut prev = vec![(u16::MAX, u16::MAX); node_to_edges.len()];
    let mut queue = VecDeque::new();

    'outer: while let Some((curr, edge)) = to_visit.pop() {
        if visited.replace(curr as usize, true) {
            continue;
        }

        edges_seen.fill(false);

        'inner: for _ in 0..4 {
            prev.fill((u16::MAX, u16::MAX));
            prev[curr as usize] = (u16::MAX - 1, u16::MAX - 1);
            queue.clear();
            for &(node, edge) in &node_to_edges[curr as usize] {
                if !edges_seen[edge as usize] {
                    prev[node as usize] = (curr, edge);
                    queue.push_back((node, edge));
                }
            }

            while let Some((mut curr, mut edge)) = queue.pop_front() {
                if set1[curr as usize] {
                    while curr != u16::MAX - 1 {
                        edges_seen.set(edge as usize, true);
                        (curr, edge) = prev[curr as usize];
                    }
                    continue 'inner;
                }
                for &(next, next_edge) in &node_to_edges[curr as usize] {
                    if !edges_seen[next_edge as usize]
                        && prev[next as usize] == (u16::MAX, u16::MAX)
                    {
                        prev[next as usize] = (curr, edge);
                        queue.push_back((next, next_edge));
                    }
                }
            }

            min_cut.set(edge as usize, true);
            min_cut_len += 1;
            if min_cut_len == 3 {
                let mut seen = FxHashSet::default();
                let mut queue = vec![0];

                while let Some(curr) = queue.pop() {
                    if seen.insert(curr) {
                        for &(node, edge) in &node_to_edges[curr as usize] {
                            if !min_cut[edge as usize] {
                                queue.push(node);
                            }
                        }
                    }
                }

                return seen.len() * (node_to_edges.len() - seen.len());
            }

            continue 'outer;
        }

        set1.set(curr as usize, true);
        for &(node, edge) in &node_to_edges[curr as usize] {
            to_visit.push((node, edge));
        }
    }

    unreachable!()
}
