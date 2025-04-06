// https://leetcode.com/problems/largest-divisible-subset/description/

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();

    let n = nums.len();
    let mut dp = vec![1; n];
    let mut prev = vec![usize::MAX; n];
    let mut max_idx = 0;

    for i in 1..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = j;
            }
            if dp[i] > dp[max_idx] {
                max_idx = i;
            }
        }
    }

    let mut result = Vec::new();
    while max_idx != usize::MAX {
        result.push(nums[max_idx]);
        max_idx = prev[max_idx];
    }

    result
}
