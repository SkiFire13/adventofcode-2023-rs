#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn visit(input: &Input, start: (usize, usize), steps: usize) -> FxHashSet<(usize, usize)> {
    let mut frontier = FxHashSet::default();
    let mut new_frontier = FxHashSet::default();

    frontier.insert(start);

    for _ in 0..steps {
        new_frontier.clear();

        for &pos in &frontier {
            new_frontier.extend(input.plus_neighbours(pos).filter(|&pos| input[pos] != b'#'));
        }

        (frontier, new_frontier) = (new_frontier, frontier);
    }

    frontier
}

pub fn part1(input: &Input) -> usize {
    let (start_pos, _) = input.iter().find(|(_, &b)| b == b'S').unwrap();
    visit(input, start_pos, 64).len()
}

pub fn part2(input: &Input) -> usize {
    let (start_pos, _) = input.iter().find(|(_, &b)| b == b'S').unwrap();

    let mut even = FxHashSet::default();
    even.insert(start_pos);
    let mut odd = FxHashSet::default();

    for _ in 0.. {
        let (even_prev, odd_prev) = (even.len(), odd.len());

        odd.clear();
        for &pos in &even {
            odd.extend(input.plus_neighbours(pos).filter(|&pos| input[pos] != b'#'));
        }

        even.clear();
        for &pos in &odd {
            even.extend(input.plus_neighbours(pos).filter(|&pos| input[pos] != b'#'));
        }

        if even.len() == even_prev && odd.len() == odd_prev {
            break;
        }
    }

    assert!(input.w() % 2 == 1);
    assert!(input.w() == input.h());
    assert!(even.contains(&(0, 0)));
    assert!(even.contains(&(input.w() - 1, 0)));
    assert!(even.contains(&(0, input.h() - 1)));
    assert!(even.contains(&(input.w() - 1, input.h() - 1)));

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

    let mut tot = if N % 2 == 0 { even.len() } else { odd.len() };

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
        tot += full_k1 * if N % 2 == 0 { even.len() } else { odd.len() };
        tot += full_k2 * if N % 2 == 0 { odd.len() } else { even.len() };
        tot += full_steps * visit(input, start, half_remainder + input.w()).len();
        tot += (full_steps + 1) * visit(input, start, half_remainder).len();
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
        tot += full_steps / 2 * if N % 2 == 0 { odd.len() } else { even.len() };
        tot += (full_steps - 1) / 2 * if N % 2 == 0 { even.len() } else { odd.len() };
        tot += visit(input, start, input.w() / 2 + half_remainder).len();
        if half_remainder > input.w() / 2 {
            tot += visit(input, start, half_remainder - input.w() / 2 - 1).len();
        }

        tot
    };

    tot += calc_straight(top);
    tot += calc_straight(bot);
    tot += calc_straight(left);
    tot += calc_straight(right);

    tot
}
