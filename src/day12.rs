#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<(&'a [u8], Vec<usize>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (pat, nums) = line.split_once(' ').unwrap();
            let pat = pat.as_bytes();
            let nums = nums.split(',').map(|n| n.parse().unwrap()).collect();
            (pat, nums)
        })
        .collect()
}

fn count(pat: &[u8], nums: impl Iterator<Item = usize>, dp: &mut Vec<usize>) -> usize {
    dp.resize(pat.len() + (pat.len() + 1) * 2, 0);
    let (streak, dp) = dp.split_at_mut(pat.len());
    let (mut cdp, mut pdp) = dp.split_at_mut(pat.len() + 1);

    let mut prev_streak = 0;
    for (&b, s) in izip!(pat, &mut *streak) {
        prev_streak = if b != b'.' { prev_streak + 1 } else { 0 };
        *s = prev_streak;
    }

    let wall_idx = pat.iter().position(|&b| b == b'#').unwrap_or(pat.len());
    pdp[..1 + wall_idx].fill(1);
    pdp[1 + wall_idx..].fill(0);

    for n in nums {
        cdp[..n].fill(0);
        cdp[n] = if streak[n - 1] == n { pdp[0] } else { 0 };
        let mut prev = cdp[n];

        for (w, &s, d, &pd) in izip!(pat.windows(n + 1), &streak[n..], &mut cdp[n + 1..], &*pdp) {
            let p1 = if w[n] != b'#' { prev } else { 0 };
            let p2 = if s >= n && w[0] != b'#' { pd } else { 0 };
            [*d, prev] = [p1 + p2; 2];
        }
        swap(&mut cdp, &mut pdp);
    }

    pdp[pat.len()]
}

pub fn part1(input: &Input) -> usize {
    let mut dp = Vec::new();
    input
        .iter()
        .map(|(pat, nums)| count(pat, nums.iter().copied(), &mut dp))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let threads = std::thread::available_parallelism().map_or(16, |n| n.get());
    input
        .par_iter()
        .with_min_len(input.len() / (2 * threads))
        .with_max_len(input.len() / threads)
        .fold(
            || (0, Vec::new(), Vec::new()),
            |(mut sum, mut dp, mut new_pat), &(pat, ref nums)| {
                new_pat.clear();
                new_pat.reserve(pat.len() * 5 + 4);
                new_pat.extend_from_slice(pat);

                for _ in 1..5 {
                    new_pat.push(b'?');
                    new_pat.extend_from_slice(pat);
                }

                let nums = itertools::repeat_n(nums, 5).flatten().copied();

                sum += count(&new_pat, nums, &mut dp);

                (sum, dp, new_pat)
            },
        )
        .map(|(sum, _, _)| sum)
        .sum()
}
