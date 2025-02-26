// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut min_sum = i32::MAX;
    let mut max_sum = i32::MIN;
    let mut prefix_sum = 0;
    let mut abs_sum = 0;

    for num in nums {
        prefix_sum += num;

        min_sum = min_sum.min(prefix_sum);
        max_sum = max_sum.max(prefix_sum);

        if prefix_sum >= 0 {
            abs_sum = abs_sum.max(prefix_sum.max(prefix_sum - min_sum));
        } else {
            abs_sum = abs_sum.max(prefix_sum.abs().max((prefix_sum - max_sum).abs()));
        }
    }

    abs_sum
}
