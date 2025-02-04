// https://leetcode.com/problems/maximum-ascending-subarray-sum/

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut longest = nums[0];
    let mut cur_sum = nums[0];
    let n = nums.len();

    for i in 1..n {
        if nums[i] > nums[i - 1] {
            cur_sum += nums[i];
        } else {
            cur_sum = nums[i];
        }
        longest = longest.max(cur_sum);
    }
    longest
}
