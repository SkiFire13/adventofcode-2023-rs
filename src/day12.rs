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

fn count(pat: &[u8], nums: &[usize], dp: &mut Vec<usize>) -> usize {
    dp.resize((pat.len() + 1) * 2, 0);
    let (mut curr_dp, mut prev_dp) = dp.split_at_mut(pat.len() + 1);

    prev_dp.fill(0);
    prev_dp[pat.len()] = 1;
    for j in (0..pat.len()).rev().take_while(|&j| pat[j] != b'#') {
        prev_dp[j] = 1;
    }

    for &n in nums.iter().rev() {
        let mut streak = 0;
        curr_dp[pat.len()] = 0;
        for (j, &p) in pat.iter().enumerate().rev() {
            streak = if p == b'.' { 0 } else { streak + 1 };
            curr_dp[j] = if p == b'#' { 0 } else { curr_dp[j + 1] };

            let prev_pat = pat.get(j.wrapping_sub(1));
            let next = pat.get(j + n);
            if streak >= n && prev_pat != Some(&b'#') && next != Some(&b'#') {
                curr_dp[j] += prev_dp[j + n + (j + n < pat.len()) as usize];
            }
        }
        swap(&mut curr_dp, &mut prev_dp);
    }

    prev_dp[0]
}

pub fn part1(input: &Input) -> usize {
    let mut dp = Vec::new();
    input
        .iter()
        .map(|(pat, nums)| count(pat, nums, &mut dp))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut dp = Vec::new();
    let mut new_pat = Vec::new();
    let mut new_nums = Vec::new();
    input
        .iter()
        .map(|&(pat, ref nums)| {
            new_pat.clear();
            new_pat.reserve(pat.len() * 5 + 4);
            new_pat.extend_from_slice(pat);

            new_nums.clear();
            new_nums.reserve(nums.len() * 5);
            new_nums.extend_from_slice(nums);

            for _ in 1..5 {
                new_pat.push(b'?');
                new_pat.extend_from_slice(pat);
                new_nums.extend_from_slice(nums);
            }

            count(&new_pat, &new_nums, &mut dp)
        })
        .sum()
}
