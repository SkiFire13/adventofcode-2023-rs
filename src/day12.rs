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
    let fillable = pat.iter().filter(|&&b| b != b'.').count();
    let to_fill = nums.iter().sum();
    return count_inner(pat, nums, fillable, to_fill, &mut FxHashMap::default());

    fn count_inner(
        pat: &[u8],
        nums: &[usize],
        mut fillable: usize,
        to_fill: usize,
        cache: &mut FxHashMap<(usize, usize), usize>,
    ) -> usize {
        if nums.len() == 0 {
            return if pat.contains(&b'#') { 0 } else { 1 };
        }

        if fillable < to_fill {
            return 0;
        }

        if let Some(&ret) = cache.get(&(pat.len(), nums.len())) {
            return ret;
        }

        let mut count = 0;

        for pos in 0..=pat.len().saturating_sub(nums[0]) {
            if pat[pos] == b'.' {
                continue;
            }

            let all_fillable = pat[pos..pos + nums[0]].iter().all(|&b| b != b'.');
            let next_not_filled = pat.get(pos + nums[0]) != Some(&b'#');

            if all_fillable && next_not_filled {
                let next_is_fillable = pat.get(pos + nums[0]) == Some(&b'?');
                let fillable = fillable - nums[0] - next_is_fillable as usize;
                let to_fill = to_fill - nums[0];
                let pat = pat.get(pos + nums[0] + 1..).unwrap_or(&[]);
                count += count_inner(pat, &nums[1..], fillable, to_fill, cache);
            }

            match pat[pos] {
                b'?' => fillable -= 1,
                b'#' => break,
                _ => {}
            }

            if fillable < to_fill {
                break;
            }
        }

        cache.insert((pat.len(), nums.len()), count);

        count
    }
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
