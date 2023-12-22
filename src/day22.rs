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
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    input.sort_by_key(|&((x, y, z), _)| (z, x, y));

    let mut maxx = 0;
    let mut maxy = 0;

    for &((x1, y1, _), (x2, y2, _)) in &input {
        maxx = max(maxx, x1);
        maxy = max(maxy, y1);
        maxx = max(maxx, x2);
        maxy = max(maxy, y2);
    }

    let mut supported_by = FxHashSet::default();

    let mut supporting = vec![false; input.len()];
    let mut last = vec![(usize::MAX, 0); (maxx + 1) * (maxy + 1)];

    for (i, &((x1, y1, z1), (x2, y2, z2))) in input.iter().enumerate() {
        let mut z_target = 0;
        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                z_target = max(z_target, last[y * (maxx + 1) + x].1)
            }
        }

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                let (s, l) = last[y * (maxx + 1) + x];
                if l == z_target && s != usize::MAX {
                    supported_by.insert(s);
                }
                last[y * (maxx + 1) + x] = (i, z_target + z2 - z1 + 1);
            }
        }
        if supported_by.len() == 1 {
            supporting[*supported_by.iter().next().unwrap()] = true;
        }
        supported_by.clear();
    }

    supporting.iter().filter(|&&sup| !sup).count()
}

pub fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    input.sort_by_key(|&((x, y, z), _)| (z, x, y));

    let mut maxx = 0;
    let mut maxy = 0;

    for &((x1, y1, _), (x2, y2, _)) in &input {
        maxx = max(maxx, x1);
        maxy = max(maxy, y1);
        maxx = max(maxx, x2);
        maxy = max(maxy, y2);
    }

    let mut under = vec![FxHashSet::default(); input.len()];
    let mut over = vec![FxHashSet::default(); input.len()];
    let mut last = vec![(usize::MAX, 0); (maxx + 1) * (maxy + 1)];

    for (i, &((x1, y1, z1), (x2, y2, z2))) in input.iter().enumerate() {
        let mut z_target = 0;
        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                z_target = max(z_target, last[y * (maxx + 1) + x].1)
            }
        }

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                let (s, l) = last[y * (maxx + 1) + x];
                if l == z_target && s != usize::MAX {
                    over[s].insert(i);
                    under[i].insert(s);
                }
                last[y * (maxx + 1) + x] = (i, z_target + z2 - z1 + 1);
            }
        }
    }

    let mut sum = 0;
    let mut queue = VecDeque::new();
    let mut seen = FxHashSet::default();
    for i in 0..input.len() {
        queue.extend(over[i].iter().copied());
        seen.clear();
        seen.insert(i);

        'outer: while let Some(brick) = queue.pop_front() {
            if seen.contains(&brick) {
                continue;
            }

            for u in &under[brick] {
                if !seen.contains(u) {
                    continue 'outer;
                }
            }

            sum += 1;
            seen.insert(brick);
            queue.extend(over[brick].iter().copied());
        }
    }
    sum
}
