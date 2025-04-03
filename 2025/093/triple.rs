// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/description/

pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut prefix_max = vec![0; n];
    let mut suffix_max = vec![0; n];
    prefix_max[0] = nums[0];
    suffix_max[n - 1] = nums[n - 1];

    for i in 1..n {
        prefix_max[i] = prefix_max[i - 1].max(nums[i]);
        suffix_max[n - i - 1] = suffix_max[n - i].max(nums[n - i - 1]);
    }

    let mut max = 0;
    for j in 1..(n - 1) {
        max = max.max((prefix_max[j - 1] - nums[j]) as i64  * suffix_max[j + 1] as i64);
    }

    max
}
