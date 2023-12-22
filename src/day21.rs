#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn reachable_in(input: &Input, start: (usize, usize), steps: usize) -> usize {
    let mut seen = bitbox![0; input.vec.len()];
    let mut seen_next = bitbox![0; input.vec.len()];

    let mut frontier = Vec::new();
    let mut frontier_next = Vec::new();

    let encode = |(x, y)| input.w() * y + x;

    seen.set(encode(start), true);
    frontier.push(start);

    for _ in 0..steps {
        for pos in frontier.drain(..) {
            for next in input.plus_neighbours(pos) {
                if input[next] != b'#' && !seen_next.replace(encode(next), true) {
                    frontier_next.push(next);
                }
            }
        }

        swap(&mut frontier, &mut frontier_next);
        swap(&mut seen, &mut seen_next);
    }

    seen.count_ones()
}

pub fn part1(input: &Input) -> usize {
    let (start_pos, _) = input.iter().find(|(_, &b)| b == b'S').unwrap();
    reachable_in(input, start_pos, 64)
}

pub fn part2(input: &Input) -> usize {
    let (start_pos, _) = input.iter().find(|(_, &b)| b == b'S').unwrap();

    let mut seen = bitbox![0; input.vec.len()];
    let mut seen_next = bitbox![0; input.vec.len()];

    let mut frontier = Vec::new();
    let mut frontier_next = Vec::new();

    let encode = |(x, y)| input.w() * y + x;

    seen.set(encode(start_pos), true);
    frontier.push(start_pos);

    while !frontier.is_empty() {
        for _ in 0..2 {
            for pos in frontier.drain(..) {
                for next in input.plus_neighbours(pos) {
                    if input[next] != b'#' && !seen_next.replace(encode(next), true) {
                        frontier_next.push(next);
                    }
                }
            }

            swap(&mut frontier, &mut frontier_next);
            swap(&mut seen, &mut seen_next);
        }
    }

    let (even, odd) = (seen.count_ones(), seen_next.count_ones());

    assert!(input.w() % 2 == 1);
    assert!(input.w() == input.h());

    let mut distances = Grid::with_dimensions_init(input.w(), input.h(), |_, _| usize::MAX);
    let mut queue = VecDeque::from([(start_pos, 0)]);
    let mut max_d = 0;

    while let Some((pos, d)) = queue.pop_front() {
        if distances[pos] <= d {
            continue;
        }
        distances[pos] = d;
        max_d = max(max_d, d);
        let valid_neighbours = input.plus_neighbours(pos).filter(|&pos| input[pos] != b'#');
        queue.extend(valid_neighbours.map(|pos| (pos, d + 1)));
    }

    assert!(max_d <= input.w());

    const N: usize = 26501365;

    let mut tot = if N % 2 == 0 { even } else { odd };

    let top_left = (0usize, 0);
    let top_right = (input.w() - 1, 0);
    let bot_left = (0, input.h() - 1);
    let bot_right = (input.w() - 1, input.h() - 1);
    let top = (input.w() / 2, 0);
    let bot = (input.w() / 2, input.h() - 1);
    let left = (0, input.h() / 2);
    let right = (input.w() - 1, input.h() / 2);

    assert!(distances[top_left] == input.w() - 1);
    assert!(distances[top_right] == input.w() - 1);
    assert!(distances[bot_left] == input.w() - 1);
    assert!(distances[bot_right] == input.w() - 1);

    assert!(distances[top] == input.w() / 2);
    assert!(distances[bot] == input.w() / 2);
    assert!(distances[left] == input.w() / 2);
    assert!(distances[right] == input.w() / 2);

    let calc_diagonal = |start: (usize, usize)| {
        let remaining = N - (input.w() + 1);

        let full_steps = remaining / input.w();
        let half_remainder = remaining - input.w() * full_steps;

        let full_k1 = (full_steps / 2).pow(2);
        let full_k2 = ((full_steps - 1) / 2) * ((full_steps - 1) / 2 + 1);

        let mut tot = 0;
        tot += full_k1 * if N % 2 == 0 { even } else { odd };
        tot += full_k2 * if N % 2 == 0 { odd } else { even };
        tot += full_steps * reachable_in(input, start, half_remainder + input.w());
        tot += (full_steps + 1) * reachable_in(input, start, half_remainder);
        tot
    };

    tot += calc_diagonal(bot_right);
    tot += calc_diagonal(bot_left);
    tot += calc_diagonal(top_right);
    tot += calc_diagonal(top_left);

    let calc_straight = |start| {
        let full_steps = N / input.w();
        let half_remainder = N - input.w() * full_steps;

        let mut tot = 0;
        tot += full_steps / 2 * if N % 2 == 0 { odd } else { even };
        tot += (full_steps - 1) / 2 * if N % 2 == 0 { even } else { odd };
        tot += reachable_in(input, start, input.w() / 2 + half_remainder);
        if half_remainder > input.w() / 2 {
            tot += reachable_in(input, start, half_remainder - input.w() / 2 - 1);
        }

        tot
    };

    tot += calc_straight(top);
    tot += calc_straight(bot);
    tot += calc_straight(left);
    tot += calc_straight(right);

    tot
}
