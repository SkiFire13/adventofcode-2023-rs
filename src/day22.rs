use aoc_helper::prelude::itertools::iproduct;

#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<((usize, usize, usize), (usize, usize, usize))>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x1, y1, z1, x2, y2, z2) = line
                .split(&[',', '~'][..])
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            ((x1, y1, z1), (x2, y2, z2))
        })
        .sorted_unstable_by_key(|&((x, y, z), _)| (z, x, y))
        .collect()
}

fn bounds(input: &Input) -> (usize, usize) {
    let (mut maxx, mut maxy) = (0, 0);
    for &(_, (x2, y2, _)) in input {
        (maxx, maxy) = (max(maxx, x2), max(maxy, y2));
    }
    (maxx, maxy)
}

pub fn part1(input: &Input) -> usize {
    let (maxx, maxy) = bounds(input);

    let mut supporting = vec![false; input.len()];
    let mut last = vec![(usize::MAX, 0); (maxx + 1) * (maxy + 1)];

    for (i, &((x1, y1, z1), (x2, y2, z2))) in input.iter().enumerate() {
        let z_target = iproduct!(x1..x2 + 1, y1..y2 + 1)
            .map(|(x, y)| last[y * (maxx + 1) + x].1)
            .max()
            .unwrap_or(0);

        let mut last_supported_by = None;
        let mut nsupporting = 0;

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                let (s, l) = last[y * (maxx + 1) + x];
                if l == z_target && s != usize::MAX {
                    if last_supported_by != Some(s) {
                        nsupporting += 1;
                        last_supported_by = Some(s);
                    }
                }
                last[y * (maxx + 1) + x] = (i, z_target + z2 - z1 + 1);
            }
        }
        if nsupporting == 1 {
            supporting[last_supported_by.unwrap()] = true;
        }
    }

    supporting.iter().filter(|&&sup| !sup).count()
}

pub fn part2(input: &Input) -> usize {
    let (maxx, maxy) = bounds(input);

    let mut under = vec![Vec::new(); input.len()];
    let mut over = vec![Vec::new(); input.len()];
    let mut last = vec![(usize::MAX, 0); (maxx + 1) * (maxy + 1)];

    for (i, &((x1, y1, z1), (x2, y2, z2))) in input.iter().enumerate() {
        let z_target = iproduct!(x1..x2 + 1, y1..y2 + 1)
            .map(|(x, y)| last[y * (maxx + 1) + x].1)
            .max()
            .unwrap_or(0);

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                let (s, l) = last[y * (maxx + 1) + x];
                if l == z_target && s != usize::MAX {
                    if !under[i].contains(&s) {
                        under[i].push(s);
                        over[s].push(i);
                    }
                }
                last[y * (maxx + 1) + x] = (i, z_target + z2 - z1 + 1);
            }
        }
    }

    // Compute post-order
    let mut post_idx = 0;
    let mut node_to_post = vec![0; input.len()];
    let mut queue = VecDeque::new();
    queue.extend((0..input.len()).filter(|&n| over[n].len() == 0));
    while let Some(n) = queue.pop_front() {
        node_to_post[n] = post_idx;
        post_idx += 1;
        for &m in &under[n] {
            node_to_post[m] += 1;
            if node_to_post[m] == over[m].len() {
                queue.push_back(m);
            }
        }
    }

    // Compute immediate dominators
    let mut idominators = vec![input.len() + 1; input.len() + 1];
    idominators[input.len()] = input.len();
    let mut changed = true;
    while mem::take(&mut changed) {
        for i in (0..input.len()).rev() {
            let new_idom_idx = under[i]
                .iter()
                .map(|&n| node_to_post[n])
                .filter(|&p| idominators[p] != input.len() + 1)
                .reduce(|mut f1, mut f2| loop {
                    match f1.cmp(&f2) {
                        Ordering::Less => f1 = idominators[f1],
                        Ordering::Greater => f2 = idominators[f2],
                        Ordering::Equal => return f1,
                    }
                })
                .unwrap_or(input.len());
            if new_idom_idx != idominators[node_to_post[i]] {
                idominators[node_to_post[i]] = new_idom_idx;
                changed = true;
            }
        }
    }

    // Compute comulative number of dominators for each node
    let mut ndominators = vec![0; input.len() + 1];
    for i in (0..input.len()).rev() {
        ndominators[i] = ndominators[idominators[i]] + 1;
    }
    ndominators.iter().sum::<usize>() - input.len()
}
