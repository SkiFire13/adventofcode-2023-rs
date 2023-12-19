#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (
    HashMap<&'a str, Vec<(usize, bool, usize, &'a str)>>,
    Vec<[usize; 4]>,
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
                        return (0, false, usize::MAX - 1, rule);
                    }
                    let rating = match rule.as_bytes()[0] {
                        b'x' => 0,
                        b'm' => 1,
                        b'a' => 2,
                        b's' => 3,
                        _ => panic!(),
                    };
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
            [x, m, a, s].map(|n| n.parse().unwrap())
        })
        .collect();
    (ratings, parts)
}

pub fn part1(input: &Input) -> usize {
    let (ratings, parts) = input;

    parts
        .iter()
        .filter(|xmas| {
            let mut rating = "in";
            loop {
                for &(c, is_greater, n, next) in &ratings[rating] {
                    let cond = match is_greater {
                        true => xmas[c] > n,
                        false => xmas[c] < n,
                    };

                    if cond {
                        match next {
                            "A" => return true,
                            "R" => return false,
                            _ => rating = next,
                        }
                        break;
                    }
                }
            }
        })
        .flatten()
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let (ratings, _) = input;

    let mut count = 0;
    let mut candidates = vec![("in", [(1, 4001); 4])];

    while let Some((rule, mut xmas)) = candidates.pop() {
        if let "A" | "R" = rule {
            if rule == "A" {
                count += xmas.iter().map(|(s, e)| e - s).product::<usize>();
            }
            continue;
        }

        for &(c, is_greater, n, next) in &ratings[rule] {
            let (c1, c2) = xmas[c];
            let n = Ord::clamp(n + is_greater as usize, c1, c2);
            let (sat, unsat) = match is_greater {
                true => ((n, c2), (c1, n)),
                false => ((c1, n), (n, c2)),
            };

            if sat.0 < sat.1 {
                let mut new = xmas;
                new[c] = sat;
                candidates.push((next, new));
            }

            if unsat.0 >= unsat.1 {
                break;
            }

            xmas[c] = unsat;
        }
    }

    count
}
