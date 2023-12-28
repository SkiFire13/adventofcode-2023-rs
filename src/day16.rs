#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut queue = vec![((0, 0), (1, 0))];
    let mut seen = Grid::<u8>::with_dimensions(input.w(), input.h());

    while let Some(((x, y), (dx, dy))) = queue.pop() {
        let Some(tile) = input.iget((x as isize, y as isize)) else {
            continue;
        };

        let dir_id = (dx + dy + 1 + (dx > dy) as i8) as u8;
        let seen_value = &mut seen[(x as isize, y as isize)];
        if *seen_value & (1 << dir_id) != 0 {
            continue;
        }
        *seen_value |= 1 << dir_id;

        match (tile, (dx, dy)) {
            (b'.', _) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'|', (0, _)) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'-', (_, 0)) => queue.push(((x + dx, y + dy), (dx, dy))),
            (b'|', (_, 0)) => queue.extend([((x, y + 1), (0, 1)), ((x, y - 1), (0, -1))]),
            (b'-', (0, _)) => queue.extend([((x + 1, y), (1, 0)), ((x - 1, y), (-1, 0))]),
            (b'/', _) => queue.push(((x - dy, y - dx), (-dy, -dx))),
            (b'\\', _) => queue.push(((x + dy, y + dx), (dy, dx))),
            _ => panic!(),
        }
    }

    seen.iter().filter(|&(_, &idx)| idx != 0).count()
}

type MirrorData = [(usize, Vec<(usize, usize)>); 2];
fn detect_mirrors(input: &Input) -> (FxHashMap<(usize, usize), usize>, Vec<MirrorData>) {
    let mut pos_to_mirror = FxHashMap::default();
    let mut mirrors = vec![[(0, Vec::new()), (0, Vec::new())]];

    for y in 0..input.h() {
        for x in 0..input.w() {
            let (dx, dy) = match input[(x, y)] {
                b'-' => (1, 0),
                b'|' => (0, 1),
                _ => continue,
            };
            let mirror_id = *pos_to_mirror.entry((x, y)).or_insert_with(|| {
                mirrors.push(<_>::default());
                mirrors.len() - 1
            });
            for (i, (mut dx, mut dy)) in [(dx, dy), (-dx, -dy)].into_iter().enumerate() {
                let mut visited = vec![(x, y)];
                let (mut x, mut y) = (x as isize + dx, y as isize + dy);
                loop {
                    (x, y, dx, dy) = match input.iget((x, y)) {
                        Some(b'-' | b'|') | None => break,
                        Some(b'.') => (x + dx, y + dy, dx, dy),
                        Some(b'/') => (x - dy, y - dx, -dy, -dx),
                        Some(b'\\') => (x + dy, y + dx, dy, dx),
                        _ => panic!(),
                    };
                    visited.push(((x - dx) as usize, (y - dy) as usize));
                }
                let (x, y) = (x as usize, y as usize);
                let next_mirror = if x < input.w() && y < input.h() {
                    *pos_to_mirror.entry((x, y)).or_insert_with(|| {
                        mirrors.push(<_>::default());
                        mirrors.len() - 1
                    })
                } else {
                    0
                };
                mirrors[mirror_id][i] = (next_mirror, visited);
            }
        }
    }

    (pos_to_mirror, mirrors)
}

fn detect_sccs(mirrors: &[MirrorData]) -> (Vec<Vec<usize>>, Vec<usize>) {
    struct Tarjan<'a> {
        next_index: usize,
        index: Vec<usize>,
        lowlink: Vec<usize>,
        onstack: Vec<bool>,
        stack: Vec<usize>,
        sccs: Vec<Vec<usize>>,
        mirrors: &'a [[(usize, Vec<(usize, usize)>); 2]],
    }

    fn tarjan(ctx: &mut Tarjan<'_>, i: usize) {
        ctx.index[i] = ctx.next_index;
        ctx.lowlink[i] = ctx.next_index;
        ctx.next_index += 1;
        ctx.onstack[i] = true;
        let stack_idx = ctx.stack.len();
        ctx.stack.push(i);
        for &(next, _) in &ctx.mirrors[i] {
            if ctx.index[next] == usize::MAX {
                tarjan(ctx, next);
                ctx.lowlink[i] = min(ctx.lowlink[i], ctx.lowlink[next]);
            } else if ctx.onstack[next] {
                ctx.lowlink[i] = min(ctx.lowlink[i], ctx.index[next]);
            }
        }
        if ctx.lowlink[i] == ctx.index[i] {
            let scc = ctx.stack.drain(stack_idx..).collect::<Vec<_>>();
            scc.iter().for_each(|&j| ctx.onstack[j] = false);
            ctx.sccs.push(scc);
        }
    }

    let mut ctx = Tarjan {
        next_index: 0,
        index: vec![usize::MAX; mirrors.len()],
        lowlink: vec![usize::MAX; mirrors.len()],
        onstack: vec![false; mirrors.len()],
        stack: Vec::new(),
        sccs: Vec::new(),
        mirrors,
    };

    for i in 0..mirrors.len() {
        if ctx.index[i] == usize::MAX {
            tarjan(&mut ctx, i);
        }
    }

    let mut scc_of = vec![0; mirrors.len()];
    for (i, scc) in ctx.sccs.iter().enumerate() {
        for &j in scc {
            scc_of[j] = i;
        }
    }

    (ctx.sccs, scc_of)
}

pub fn part2(input: &Input) -> usize {
    let (pos_to_mirror, mirrors) = detect_mirrors(input);
    let (sccs, scc_of) = detect_sccs(&mirrors);

    let mut scc_size = vec![0; sccs.len()];
    let mut scc_reach = vec![BitBox::default(); sccs.len()];
    let mut scc_visit = vec![BitBox::default(); sccs.len()];
    for (i, scc) in sccs.iter().enumerate() {
        let mut visit = bitbox![0; input.vec.len()];
        let mut reach = bitbox![0; sccs.len()];
        reach.set(i, true);
        for &curr in scc {
            for &(next, ref visited) in &mirrors[curr] {
                for (x, y) in visited {
                    visit.set(y * input.w() + x, true);
                }
                if !reach.replace(scc_of[next], true) {
                    reach |= &scc_reach[scc_of[next]];
                    visit |= &scc_visit[scc_of[next]];
                }
            }
        }
        scc_size[i] = visit.count_ones();
        scc_visit[i] = visit;
        scc_reach[i] = reach;
    }

    let mut visited = Vec::new();
    let (w, h) = (input.w(), input.h());
    itertools::chain!(
        (0..w).map(|x| ((x, 0), (0, 1))),
        (0..w).map(|x| ((x, h - 1), (0, -1))),
        (0..h).map(|y| ((0, y), (1, 0))),
        (0..h).map(|y| ((w - 1, y), (-1, 0)))
    )
    .map(|((x, y), (mut dx, mut dy))| {
        visited.clear();
        let (mut x, mut y) = (x as isize, y as isize);
        loop {
            (x, y, dx, dy) = match input.iget((x, y)) {
                Some(b'-' | b'|') | None => break,
                Some(b'.') => (x + dx, y + dy, dx, dy),
                Some(b'/') => (x - dy, y - dx, -dy, -dx),
                Some(b'\\') => (x + dy, y + dx, dy, dx),
                _ => panic!(),
            };
            visited.push(((x - dx) as usize, (y - dy) as usize));
        }
        if 0 <= x && x < input.w() as isize && 0 <= y && y < input.h() as isize {
            let scc = scc_of[pos_to_mirror[&(x as usize, y as usize)]];
            let visit = &scc_visit[scc];
            let new = visited
                .iter()
                .filter(|(x, y)| !visit[input.w() * y + x])
                .count();
            scc_size[scc] + new
        } else {
            visited.len()
        }
    })
    .max()
    .unwrap()
}
