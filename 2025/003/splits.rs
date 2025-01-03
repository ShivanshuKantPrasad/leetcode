// https://leetcode.com/problems/number-of-ways-to-split-array/description/

pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prefix_sum = vec![0; n];
    prefix_sum[0] = nums[0] as i64;

    for i in 1..n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
    }

    let mut count = 0;
    for i in 0..(n - 1) {
        let left_sum = prefix_sum[i];
        let right_sum = prefix_sum[n - 1] - prefix_sum[i];
        if left_sum >= right_sum {
            count += 1;
        }
    }
    count
}
