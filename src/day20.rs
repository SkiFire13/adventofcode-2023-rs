#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (
    Vec<&'a str>,
    FxHashMap<&'a str, Vec<&'a str>>,
    FxHashMap<&'a str, Vec<&'a str>>,
);

pub fn input_generator(input: &str) -> Input {
    let mut broadcaster = Vec::new();
    let mut inv = FxHashMap::default();
    let mut conj = FxHashMap::default();
    for line in input.lines() {
        let (key, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").collect();
        if key == "broadcaster" {
            broadcaster = outputs;
        } else if key.starts_with("%") {
            inv.insert(&key[1..], outputs);
        } else {
            assert!(key.starts_with("&"));
            conj.insert(&key[1..], outputs);
        }
    }
    (broadcaster, inv, conj)
}

pub fn part1(input: &Input) -> usize {
    let (broadcaster, inv, conj) = input;

    let mut inv_states = inv
        .keys()
        .map(|&key| (key, false))
        .collect::<FxHashMap<_, _>>();
    let mut conj_states = conj
        .keys()
        .map(|&key| (key, FxHashMap::default()))
        .collect::<FxHashMap<_, _>>();
    let mut conj_states_count = conj
        .keys()
        .map(|&key| (key, 0))
        .collect::<FxHashMap<_, _>>();

    for &output in broadcaster {
        if let Some(conj_state) = conj_states.get_mut(output) {
            conj_state.insert("broadcaster", false);
        }
    }
    for (key, outputs) in inv {
        for output in outputs {
            if let Some(conj_state) = conj_states.get_mut(output) {
                conj_state.insert(key, false);
            }
        }
    }
    for (key, outputs) in conj {
        for output in outputs {
            if let Some(conj_state) = conj_states.get_mut(output) {
                conj_state.insert(key, false);
            }
        }
    }

    // println!("{:?}", inv_states);
    // println!("{:?}", conj_states);
    // println!("{:?}", conj_states_count);
    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1000 {
        let mut queue = VecDeque::from([("button", "broadcaster", false)]);
        while let Some((prev, curr, level)) = queue.pop_front() {
            // println!("{prev} -{level}-> {curr}");
            if level {
                highs += 1;
            } else {
                lows += 1;
            }
            if curr == "broadcaster" {
                queue.extend(broadcaster.iter().map(|&next| (curr, next, level)));
            } else if inv.contains_key(curr) {
                if !level {
                    let state = inv_states.entry(curr).or_default();
                    *state = !*state;
                    queue.extend(inv[curr].iter().map(|&next| (curr, next, *state)));
                }
            } else if conj.contains_key(curr) {
                assert!(conj_states.contains_key(curr));
                assert!(conj_states_count.contains_key(curr));
                let entry = conj_states.get_mut(curr).unwrap().get_mut(prev).unwrap();
                let before = *entry;
                *entry = level;
                if level {
                    if !before {
                        *conj_states_count.get_mut(curr).unwrap() += 1;
                    }
                    let signal = conj_states_count[curr] != conj_states[curr].len();
                    queue.extend(conj[curr].iter().map(|&next| (curr, next, signal)));
                } else {
                    if before {
                        *conj_states_count.get_mut(curr).unwrap() -= 1;
                    }
                    queue.extend(conj[curr].iter().map(|&next| (curr, next, true)));
                }
            }
        }
        // println!("{:?}", inv_states);
        // println!("{:?}", conj_states);
        // println!("{:?}", conj_states_count);
    }

    lows * highs
}

pub fn part2(input: &Input) -> usize {
    let (broadcaster, inv, conj) = input;

    let mut inv_states = inv
        .keys()
        .map(|&key| (key, false))
        .collect::<FxHashMap<_, _>>();
    let mut conj_states = conj
        .keys()
        .map(|&key| (key, FxHashMap::default()))
        .collect::<FxHashMap<_, _>>();
    let mut conj_states_count = conj
        .keys()
        .map(|&key| (key, 0))
        .collect::<FxHashMap<_, _>>();

    for &output in broadcaster {
        if let Some(conj_state) = conj_states.get_mut(output) {
            conj_state.insert("broadcaster", false);
        }
    }
    for (key, outputs) in inv {
        for output in outputs {
            if let Some(conj_state) = conj_states.get_mut(output) {
                conj_state.insert(key, false);
            }
        }
    }
    for (key, outputs) in conj {
        for output in outputs {
            if let Some(conj_state) = conj_states.get_mut(output) {
                conj_state.insert(key, false);
            }
        }
    }

    let (&rx_prev, _) = conj
        .iter()
        .find(|&(_, outputs)| outputs.contains(&"rx"))
        .unwrap();
    let mut rx_prev_prevs = conj
        .iter()
        .filter(|&(_, outputs)| outputs.contains(&rx_prev))
        .map(|(&key, _)| (key, 0))
        .collect::<FxHashMap<_, _>>();
    let mut rx_prev_prevs_seen = 0;

    for i in 1.. {
        let mut queue = VecDeque::from([("button", "broadcaster", false)]);
        while let Some((prev, curr, level)) = queue.pop_front() {
            if level && curr == rx_prev {
                if rx_prev_prevs[prev] == 0 {
                    rx_prev_prevs.insert(prev, i);
                    rx_prev_prevs_seen += 1;
                    if rx_prev_prevs_seen == rx_prev_prevs.len() {
                        return rx_prev_prevs.values().product();
                    }
                }
            }

            if !level && curr == "rx" {
                return i;
            }
            if curr == "broadcaster" {
                queue.extend(broadcaster.iter().map(|&next| (curr, next, level)));
            } else if inv.contains_key(curr) {
                if !level {
                    let state = inv_states.entry(curr).or_default();
                    *state = !*state;
                    queue.extend(inv[curr].iter().map(|&next| (curr, next, *state)));
                }
            } else if conj.contains_key(curr) {
                assert!(conj_states.contains_key(curr));
                assert!(conj_states_count.contains_key(curr));
                let entry = conj_states.get_mut(curr).unwrap().get_mut(prev).unwrap();
                let before = *entry;
                *entry = level;
                if level {
                    if !before {
                        *conj_states_count.get_mut(curr).unwrap() += 1;
                    }
                    let signal = conj_states_count[curr] != conj_states[curr].len();
                    queue.extend(conj[curr].iter().map(|&next| (curr, next, signal)));
                } else {
                    if before {
                        *conj_states_count.get_mut(curr).unwrap() -= 1;
                    }
                    queue.extend(conj[curr].iter().map(|&next| (curr, next, true)));
                }
            }
        }
    }

    unreachable!()
}
