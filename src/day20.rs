#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(Vec<u8>, u8, Vec<u8>)>;

pub fn input_generator(input: &str) -> Input {
    let mut nodes = Input::with_capacity(64);
    let mut name_to_idx = FxHashMap::default();
    name_to_idx.reserve(64);

    nodes.push(<_>::default());
    nodes.push(<_>::default());
    name_to_idx.insert(0, 0);
    name_to_idx.insert(u16::from_ne_bytes(*b"rx"), 1);

    for line in input.lines() {
        let mut get_idx_of = |name| {
            let next_idx = name_to_idx.len();
            let name = u16::from_ne_bytes(name);
            *name_to_idx.entry(name).or_insert_with(|| {
                nodes.push(<_>::default());
                next_idx
            }) as u8
        };

        let (key, post) = line.split_once(" -> ").unwrap();

        let key = match key {
            "broadcaster" => 0,
            _ => get_idx_of(key.as_bytes()[1..3].try_into().unwrap()),
        };

        let post = post
            .split(", ")
            .map(|name| get_idx_of(name.as_bytes().try_into().unwrap()))
            .collect();
        for &output in &post {
            nodes[output as usize].0.push(key);
        }

        nodes[key as usize].1 = line.as_bytes()[0];
        nodes[key as usize].2 = post;
    }
    nodes
}

pub fn part1(input: &Input) -> usize {
    let (mut lows, mut highs) = (0, 0);

    let mut inv_states = vec![false; input.len()];
    let mut conj_low_counts = vec![0; input.len()];
    let mut conj_states = input
        .iter()
        .enumerate()
        .filter(|&(_, &(_, kind, _))| kind == b'&')
        .flat_map(|(key, (pre, _, _))| pre.iter().map(move |&prev| ((key as u8, prev), false)))
        .collect::<FxHashMap<_, _>>();

    let mut queue = VecDeque::new();
    for _ in 1..1001 {
        queue.push_back((0, false, 0));
        while let Some((prev, is_high, curr)) = queue.pop_front() {
            match is_high {
                true => highs += 1,
                false => lows += 1,
            }

            let (pre, kind, post) = &input[curr as usize];
            match kind {
                b'b' => queue.extend(post.iter().map(|&next| (curr, is_high, next))),
                b'%' => {
                    if !is_high {
                        let out_is_high = &mut inv_states[curr as usize];
                        *out_is_high = !*out_is_high;
                        queue.extend(post.iter().map(|&next| (curr, *out_is_high, next)));
                    }
                }
                b'&' => {
                    let count = &mut conj_low_counts[curr as usize];
                    let prev_is_high = conj_states.get_mut(&(curr, prev)).unwrap();

                    match (is_high, *prev_is_high) {
                        (true, false) => *count += 1,
                        (false, true) => *count -= 1,
                        _ => {}
                    }

                    *prev_is_high = is_high;
                    let out_high = *count != pre.len();
                    queue.extend(post.iter().map(|&next| (curr, out_high, next)));
                }
                0 => {}
                _ => unreachable!(),
            }
        }
    }

    lows * highs
}

pub fn part2(input: &Input) -> usize {
    let (_, _, post_broadcaster) = &input[0];
    let (rx_pre, _, _) = &input[1];
    let (rx_pre_pre, _, _) = &input[rx_pre[0] as usize];

    let mut cycles = 1;

    for &post_center in rx_pre_pre {
        let (center, _, _) = &input[post_center as usize];
        let center = center[0];

        let &(mut curr) = post_broadcaster
            .iter()
            .filter(|&&i| input[i as usize].2.contains(&center))
            .exactly_one()
            .unwrap();

        let mut acc = 0;
        for i in 0.. {
            let (_, _, next) = &input[curr as usize];
            acc |= (next.contains(&center) as usize) << i;
            match next.iter().filter(|&&i| i != center).exactly_one() {
                Ok(&next) => curr = next,
                Err(_) => break,
            }
        }
        cycles *= acc;
    }

    cycles
}
