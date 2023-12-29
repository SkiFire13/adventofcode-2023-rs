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
    dp.resize((pat.len() + 2) * 2, 0);
    let (mut curr_dp, mut prev_dp) = dp.split_at_mut(pat.len() + 2);

    let idx = pat.iter().position(|&b| b == b'#').unwrap_or(pat.len());
    prev_dp[..2 + idx].fill(1);
    prev_dp[2 + idx..].fill(0);

    for n in nums {
        let mut streak = 0;
        curr_dp[0] = 0;
        curr_dp[1] = 0;
        for (j, &p) in pat.iter().enumerate() {
            streak = if p == b'.' { 0 } else { streak + 1 };
            curr_dp[2 + j] = if p == b'#' { 0 } else { curr_dp[1 + j] };

            let prev = pat.get(j.wrapping_sub(n));
            let next = pat.get(j + 1);
            if streak >= n && prev != Some(&b'#') && next != Some(&b'#') {
                curr_dp[2 + j] += prev_dp[2 + j - n - 1];
            }
        }
        swap(&mut curr_dp, &mut prev_dp);
    }

    prev_dp[pat.len() + 1]
}

pub fn part1(input: &Input) -> usize {
    let mut dp = Vec::new();
    input
        .iter()
        .map(|(pat, nums)| count(pat, nums.iter().copied(), &mut dp))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .par_iter()
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
