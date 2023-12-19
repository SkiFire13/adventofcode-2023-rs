#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (
    HashMap<&'a str, Vec<(char, bool, usize, &'a str)>>,
    Vec<(usize, usize, usize, usize)>,
);

pub fn input_generator(input: &str) -> Input {
    let (ratings, parts) = input.split_once("\n\n").unwrap();
    let ratings = ratings
        .lines()
        .map(|line| {
            let (name, rest) = line[..line.len() - 1].split_once('{').unwrap();
            let rules = rest
                .split(',')
                .map(|rule| {
                    if !rule.contains(':') {
                        return ('x', false, usize::MAX - 1, rule);
                    }
                    let rating = rule.as_bytes()[0] as char;
                    let is_greater = rule.as_bytes()[1] == b'>';
                    let (num, next) = rule[2..].split_once(':').unwrap();
                    let num = num.parse::<usize>().unwrap();
                    (rating, is_greater, num, next)
                })
                .collect();
            (name, rules)
        })
        .collect();
    let parts = parts
        .lines()
        .map(|line| {
            let (_, x, _, m, _, a, _, s, _) =
                line.split(&['=', ',', '}'][..]).collect_tuple().unwrap();
            let x = x.parse().unwrap();
            let m = m.parse().unwrap();
            let a = a.parse().unwrap();
            let s = s.parse().unwrap();
            (x, m, a, s)
        })
        .collect();
    (ratings, parts)
}

pub fn part1(input: &Input) -> usize {
    let (ratings, parts) = input;

    parts
        .iter()
        .filter(|&&(x, m, a, s)| {
            let mut curr = &ratings["in"];
            'outer: loop {
                for &(c, is_greater, n, next) in curr {
                    let c = match c {
                        'x' => x,
                        'm' => m,
                        'a' => a,
                        's' => s,
                        _ => panic!(),
                    };

                    let cond = match is_greater {
                        true => c > n,
                        false => c < n,
                    };

                    if cond {
                        match next {
                            "A" => return true,
                            "R" => return false,
                            _ => {
                                curr = &ratings[next];
                                continue 'outer;
                            }
                        }
                    }
                }
                panic!()
            }
        })
        .map(|&(x, m, a, s)| x + m + a + s)
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let (ratings, _) = input;

    let mut count = 0;
    let mut candidates = vec![("in", ((1, 4001), (1, 4001), (1, 4001), (1, 4001)))];

    'cand: while let Some((rule, (mut x, mut m, mut a, mut s))) = candidates.pop() {
        if rule == "A" {
            let (x1, x2) = x;
            let (m1, m2) = m;
            let (a1, a2) = a;
            let (s1, s2) = s;
            count += (x2 - x1) * (m2 - m1) * (a2 - a1) * (s2 - s1);
            continue;
        }

        if rule == "R" {
            continue;
        }

        for &(c, is_greater, n, next) in &ratings[rule] {
            let (c1, c2) = match c {
                'x' => x,
                'm' => m,
                'a' => a,
                's' => s,
                _ => panic!(),
            };

            let (sat, unsat) = match is_greater {
                true => {
                    let n = Ord::clamp(n + 1, c1, c2);
                    ((n, c2), (c1, n))
                }
                false => {
                    let n = Ord::clamp(n, c1, c2);
                    ((c1, n), (n, c2))
                }
            };

            if sat.0 < sat.1 {
                let new = match c {
                    'x' => (sat, m, a, s),
                    'm' => (x, sat, a, s),
                    'a' => (x, m, sat, s),
                    's' => (x, m, a, sat),
                    _ => panic!(),
                };
                candidates.push((next, new));
            }

            if unsat.0 >= unsat.1 {
                continue 'cand;
            }

            match c {
                'x' => x = unsat,
                'm' => m = unsat,
                'a' => a = unsat,
                's' => s = unsat,
                _ => panic!(),
            }
        }

        panic!()
    }

    count
}
