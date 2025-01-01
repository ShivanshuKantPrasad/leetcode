// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/description/

pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    const MODULO: i32 = 1000_000_000 + 7;
    let (n, left, right) = (n as usize, left as usize, right as usize);
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }
    let mut sums = Vec::new();
    for i in 0..n {
        for j in i + 1..=n {
            sums.push(prefix_sum[j] - prefix_sum[i]);
        }
    }
    sums.sort_unstable();
    sums[left - 1..right]
        .into_iter()
        .fold(0, |acc, e| (acc + e) % MODULO)
}
