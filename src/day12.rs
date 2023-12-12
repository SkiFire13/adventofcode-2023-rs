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

fn count(pat: &[u8], nums: &[usize]) -> usize {
    let mut dp = vec![0; (pat.len() + 1) * (nums.len() + 1)];

    let offset = (pat.len() + 1) * nums.len();
    dp[offset + pat.len()] = 1;
    for j in (0..pat.len()).rev().take_while(|&j| pat[j] != b'#') {
        dp[offset + j] = 1;
    }

    for i in (0..nums.len()).rev() {
        let mut streak = 0;
        for j in (0..pat.len()).rev() {
            streak += 1;
            if pat[j] == b'.' {
                streak = 0;
            }

            if pat[j] != b'#' {
                dp[(pat.len() + 1) * i + j] = dp[(pat.len() + 1) * i + j + 1];
            }

            let prev = j.checked_sub(1).map(|j| pat[j]).unwrap_or(b'.');
            let next = pat.get(j + nums[i]).copied().unwrap_or(b'.');
            if streak >= nums[i] && prev != b'#' && next != b'#' {
                let next_idx = j + nums[i] + (j + nums[i] < pat.len()) as usize;
                dp[(pat.len() + 1) * i + j] += dp[(pat.len() + 1) * (i + 1) + next_idx];
            }
        }
    }

    dp[0]
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|(pat, nums)| count(pat, nums)).sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|(pat, nums)| {
            let mut new_pat = pat.to_vec();
            let mut new_nums = nums.to_vec();

            for _ in 1..5 {
                new_pat.push(b'?');
                new_pat.extend_from_slice(pat);
                new_nums.extend_from_slice(nums);
            }

            count(&new_pat, &new_nums)
        })
        .sum()
}
