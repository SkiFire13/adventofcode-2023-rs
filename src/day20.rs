#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = FxHashMap<&'a str, (Vec<&'a str>, u8, Vec<&'a str>)>;

pub fn input_generator(input: &str) -> Input {
    let mut map = Input::default();
    for line in input.lines() {
        let (key, post) = line.split_once(" -> ").unwrap();
        let post = post.split(", ").collect();

        let kind = key.as_bytes()[0];
        let key = &key[1..];

        for &output in &post {
            map.entry(output).or_default().0.push(key);
        }

        let entry = map.entry(key).or_default();
        entry.1 = kind;
        entry.2 = post;
    }
    map
}

fn solve(input: &Input, mut cb: impl FnMut(usize, &str, bool, &str) -> ControlFlow<()>) {
    let mut inv_states = input
        .iter()
        .filter(|&(_, &(_, kind, _))| kind == b'%')
        .map(|(&key, _)| (key, false))
        .collect::<FxHashMap<_, _>>();
    let mut conj_states = input
        .iter()
        .filter(|&(_, &(_, kind, _))| kind == b'&')
        .map(|(&key, (pre, _, _))| {
            let prevs_state = pre
                .iter()
                .map(|&prev| (prev, false))
                .collect::<FxHashMap<_, _>>();
            (key, (0, prevs_state))
        })
        .collect::<FxHashMap<_, _>>();

    for i in 1.. {
        let mut queue = VecDeque::from([("button", false, "roadcaster")]);
        while let Some((prev, is_high, curr)) = queue.pop_front() {
            if cb(i, prev, is_high, curr).is_break() {
                return;
            }

            let (_, kind, post) = &input[curr];
            match kind {
                b'b' => queue.extend(post.iter().map(|&next| (curr, is_high, next))),
                b'%' => {
                    if !is_high {
                        let out_is_high = inv_states.entry(curr).or_default();
                        *out_is_high = !*out_is_high;
                        queue.extend(post.iter().map(|&next| (curr, *out_is_high, next)));
                    }
                }
                b'&' => {
                    let (count, prevs_state) = conj_states.get_mut(curr).unwrap();
                    let prev_is_high = prevs_state.get_mut(prev).unwrap();

                    match (is_high, *prev_is_high) {
                        (true, false) => *count += 1,
                        (false, true) => *count -= 1,
                        _ => {}
                    }

                    *prev_is_high = is_high;
                    let out_high = *count != prevs_state.len();
                    queue.extend(post.iter().map(|&next| (curr, out_high, next)));
                }
                0 => {}
                _ => unreachable!(),
            }
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let (mut lows, mut highs) = (0, 0);

    solve(input, |i, _, is_high, _| {
        if i == 1001 {
            return ControlFlow::Break(());
        }

        match is_high {
            true => highs += 1,
            false => lows += 1,
        }

        ControlFlow::Continue(())
    });

    lows * highs
}

pub fn part2(input: &Input) -> usize {
    let (rx_pre, _, _) = &input["rx"];
    assert_eq!(rx_pre.len(), 1);
    let (rx_pre_pre, rx_pre_kind, _) = &input[&rx_pre[0]];
    assert_eq!(rx_pre_kind, &b'&');
    let mut rx_pre_pre = rx_pre_pre
        .iter()
        .map(|&prev| (prev, 0))
        .collect::<FxHashMap<_, _>>();
    let mut rx_pre_pre_seen = 0;

    solve(input, |i, prev, is_high, curr| {
        if is_high && curr == rx_pre[0] {
            if let Some(prev_iter @ 0) = rx_pre_pre.get_mut(prev) {
                *prev_iter = i;
                rx_pre_pre_seen += 1;
                if rx_pre_pre_seen == rx_pre_pre.len() {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    });

    return rx_pre_pre.values().product();
}
