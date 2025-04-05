// https://leetcode.com/problems/sum-of-all-subset-xor-totals/
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, x| a | x) << (nums.len() - 1)
}
