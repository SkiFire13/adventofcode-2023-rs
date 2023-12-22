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
        let z_target = iproduct!(y1..y2 + 1, x1..x2 + 1)
            .map(|(y, x)| last[y * (maxx + 1) + x].1)
            .max()
            .unwrap_or(0);

        let mut last_supported_by = None;
        let mut nsupporting = 0;

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
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
    let mut last = vec![(usize::MAX, 0); (maxx + 1) * (maxy + 1)];

    let mut under = Vec::with_capacity(4);

    let mut idoms = vec![input.len() + 1; input.len() + 1];
    let mut ndoms = vec![0; input.len() + 1];
    idoms[0] = 0;

    let mut sum = 0;

    for (i, &((x1, y1, z1), (x2, y2, z2))) in input.iter().enumerate() {
        let z_target = iproduct!(y1..y2 + 1, x1..x2 + 1)
            .map(|(y, x)| last[y * (maxx + 1) + x].1)
            .max()
            .unwrap_or(0);

        under.clear();
        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                let (s, l) = last[y * (maxx + 1) + x];
                if l == z_target && s != usize::MAX {
                    if !under.contains(&s) {
                        under.push(s);
                    }
                }
                last[y * (maxx + 1) + x] = (i, z_target + z2 - z1 + 1);
            }
        }

        idoms[i + 1] = under
            .iter()
            .map(|&n| n + 1)
            .reduce(|mut f1, mut f2| loop {
                match f1.cmp(&f2) {
                    Ordering::Less => f2 = idoms[f2],
                    Ordering::Greater => f1 = idoms[f1],
                    Ordering::Equal => return f1,
                }
            })
            .unwrap_or(0);
        ndoms[i + 1] = ndoms[idoms[i + 1]] + 1;
        sum += ndoms[i + 1];
    }

    sum - input.len()
}
