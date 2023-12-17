#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

fn solve<const MIN: usize, const MAX: usize, const BUCKETS: usize>(input: &Input) -> usize {
    let mut seen = bitbox![0; input.w() * input.h() * 2];
    let mut queues = array::from_fn::<_, BUCKETS, _>(|_| Vec::new());
    queues[0] = vec![(0, 0, true), (0, 0, false)];

    let mut loss = 0;

    loop {
        let mut queue = std::mem::take(&mut queues[loss % BUCKETS]);
        for (x, y, vert) in queue.drain(..) {
            let (x, y) = (x as usize, y as usize);
            if x == input.w() - 1 && y == input.h() - 1 {
                return loss;
            }

            let key = y * input.w() + x + input.vec.len() * vert as usize;
            if seen.replace(key, true) {
                continue;
            }

            let (bdx, bdy) = if vert { (0, 1) } else { (1, 0) };
            for (dx, dy) in [(bdx, bdy), (-bdx, -bdy)] {
                let (mut x, mut y, mut loss) = (x as isize, y as isize, loss);
                for i in 1..MAX + 1 {
                    (x, y) = (x + dx, y + dy);
                    let Some(&cost) = input.iget((x, y)) else {
                        break;
                    };
                    loss = loss + cost as usize;
                    if i >= MIN {
                        queues[loss % BUCKETS].push((x as u8, y as u8, !vert));
                    }
                }
            }
        }
        queues[loss % BUCKETS] = queue;
        loss += 1;
    }
}

pub fn part1(input: &Input) -> usize {
    solve::<1, 3, { (9 * 3usize + 1).next_power_of_two() }>(input)
}

pub fn part2(input: &Input) -> usize {
    solve::<4, 10, { (9 * 10usize + 1).next_power_of_two() }>(input)
}
