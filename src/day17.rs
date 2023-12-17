#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8 - b'0')
}

fn solve(input: &Input, steps: std::ops::Range<usize>) -> usize {
    let mut seen = bitbox![0; input.w() * input.h() * 2];
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0, true), (Reverse(0), 0, 0, false)]);

    while let Some((Reverse(loss), x, y, vert)) = queue.pop() {
        if x == input.w() as isize - 1 && y == input.h() as isize - 1 {
            return loss;
        }

        let key = y as usize * input.w() + x as usize + input.vec.len() * vert as usize;
        if seen.replace(key, true) {
            continue;
        }

        let (bdx, bdy) = if vert { (0, 1) } else { (1, 0) };
        for (dx, dy) in [(bdx, bdy), (-bdx, -bdy)] {
            let (mut x, mut y, mut loss) = (x, y, loss);
            for i in 1..steps.end {
                (x, y) = (x + dx, y + dy);
                let Some(&cost) = input.iget((x, y)) else {
                    break;
                };
                loss = loss + cost as usize;
                if i >= steps.start {
                    queue.push((Reverse(loss), x, y, !vert));
                }
            }
        }
    }

    unreachable!()
}

pub fn part1(input: &Input) -> usize {
    solve(input, 1..(3 + 1))
}

pub fn part2(input: &Input) -> usize {
    solve(input, 4..(10 + 1))
}
