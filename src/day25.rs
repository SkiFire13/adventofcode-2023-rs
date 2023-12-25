#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<Vec<(usize, usize)>>, usize);

pub fn input_generator(input: &str) -> Input {
    let mut node_to_id = FxHashMap::default();
    let mut edges = Vec::new();
    let mut next_edge_id = 0;

    for line in input.lines() {
        let (key, rest) = line.split_once(": ").unwrap();

        let key_id = *node_to_id.entry(key).or_insert_with(|| {
            edges.push(Vec::new());
            edges.len() - 1
        });

        for other in rest.split(' ') {
            let other_id = *node_to_id.entry(other).or_insert_with(|| {
                edges.push(Vec::new());
                edges.len() - 1
            });

            edges[key_id].push((other_id, next_edge_id));
            edges[other_id].push((key_id, next_edge_id));
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
    let mut to_visit = Vec::<(usize, usize)>::new();
    let mut visited = bitbox![0; node_to_edges.len()];

    set1.set(0, true);
    visited.set(0, true);
    to_visit.extend(&node_to_edges[0]);

    let mut edges_seen = bitbox![0; edge_count];
    let mut prev = vec![(usize::MAX, usize::MAX); node_to_edges.len()];
    let mut queue = VecDeque::new();

    'outer: while let Some((curr, edge)) = to_visit.pop() {
        if visited.replace(curr, true) {
            continue;
        }

        edges_seen.fill(false);

        'inner: for _ in 0..4 {
            prev.fill((usize::MAX, usize::MAX));
            prev[curr] = (usize::MAX - 1, usize::MAX - 1);
            queue.clear();
            for &(node, edge) in &node_to_edges[curr] {
                if !edges_seen[edge] {
                    prev[node] = (curr, edge);
                    queue.push_back((node, edge));
                }
            }

            while let Some((mut curr, mut edge)) = queue.pop_front() {
                if set1[curr] {
                    while curr != usize::MAX - 1 {
                        edges_seen.set(edge, true);
                        (curr, edge) = prev[curr];
                    }
                    continue 'inner;
                }
                for &(next, next_edge) in &node_to_edges[curr] {
                    if !edges_seen[next_edge] && prev[next] == (usize::MAX, usize::MAX) {
                        prev[next] = (curr, edge);
                        queue.push_back((next, next_edge));
                    }
                }
            }

            min_cut.set(edge, true);
            min_cut_len += 1;
            if min_cut_len == 3 {
                let mut prod = 1;
                for start in [0, curr] {
                    let mut seen = FxHashSet::default();
                    let mut queue = vec![start];

                    while let Some(curr) = queue.pop() {
                        if seen.insert(curr) {
                            for &(node, edge) in &node_to_edges[curr] {
                                if !min_cut[edge] {
                                    queue.push(node);
                                }
                            }
                        }
                    }

                    prod *= seen.len();
                }
                return prod;
            }

            continue 'outer;
        }

        set1.set(curr, true);
        for &(node, edge) in &node_to_edges[curr] {
            to_visit.push((node, edge));
        }
    }

    unreachable!()
}
