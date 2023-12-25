#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<usize>>;

pub fn input_generator(input: &str) -> Input {
    let mut node_to_id = FxHashMap::default();
    let mut edges = Vec::new();

    // Nodes connected to the critical edges
    // Carefully chosen.
    node_to_id.insert("vzb", 0);
    node_to_id.insert("krx", 1);
    node_to_id.insert("tqn", 2);
    edges.push(Vec::new());
    edges.push(Vec::new());
    edges.push(Vec::new());

    // One node per side
    node_to_id.insert("sbk", 3);
    node_to_id.insert("xmh", 4);
    edges.push(Vec::new());
    edges.push(Vec::new());

    for line in input.lines() {
        let (key, rest) = line.split_once(": ").unwrap();

        let key_id = *node_to_id.entry(key).or_insert_with(|| {
            edges.push(Vec::new());
            edges.len() - 1
        });

        // Don't add edges connected to the critical nodes.
        if key_id < 3 {
            continue;
        }

        for other in rest.split(' ') {
            let other_id = *node_to_id.entry(other).or_insert_with(|| {
                edges.push(Vec::new());
                edges.len() - 1
            });

            // Don't add edges connected to the critical nodes.
            if other_id < 3 {
                continue;
            }

            edges[key_id].push(other_id);
            edges[other_id].push(key_id);
        }
    }
    edges
}

fn dfs(input: &Input, start: usize) -> usize {
    let mut seen = FxHashSet::default();
    let mut queue = vec![start];

    while let Some(curr) = queue.pop() {
        if seen.insert(curr) {
            queue.extend(&input[curr]);
        }
    }

    seen.len()
}

pub fn part1(input: &Input) -> usize {
    (dfs(input, 3) + 1) * (dfs(input, 4) + 2)
}
