// https://leetcode.com/problems/minimum-index-of-a-valid-split/

pub fn minimum_index(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::<i32, usize>::new();

    for &num in &nums {
        *freq.entry(num).or_default() += 1;
    }

    let n = nums.len();
    let mut dominant = 0;
    let mut dominant_count = 0;

    for pair in freq {
        if pair.1 > n / 2 {
            dominant = pair.0;
            dominant_count = pair.1;
        }
    }

    let mut left_count = 0;
    let mut left_length = 0;
    let mut right_count = dominant_count;
    let mut right_length = n;

    for &num in &nums {
        left_length += 1;
        right_length -= 1;
        if num == dominant {
            left_count += 1;
            right_count -= 1;
        }

        if left_count > left_length / 2 && right_count > right_length / 2 {
            return left_length - 1;
        }
    }

    -1
}
