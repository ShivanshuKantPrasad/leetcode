// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/description/

pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut max_sum = 0;

    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    let mut left_max_index = vec![0; n];
    let mut right_max_index = vec![0; n];

    let mut result = vec![0; 3];

    let mut current_max_sum = prefix_sum[k] - prefix_sum[0];
    for i in k..n {
        if prefix_sum[i + 1] - prefix_sum[i + 1 - k] > current_max_sum {
            left_max_index[i] = i + 1 - k;
            current_max_sum = prefix_sum[i + 1] - prefix_sum[i + 1 - k];
        } else {
            left_max_index[i] = left_max_index[i - 1];
        }
    }

    right_max_index[n - k] = n - k;
    current_max_sum = prefix_sum[n] - prefix_sum[n - k];
    for i in (0..(n - k)).rev() {
        if prefix_sum[i + k] - prefix_sum[i] >= current_max_sum {
            right_max_index[i] = i;
            current_max_sum = prefix_sum[i + k] - prefix_sum[i];
        } else {
            right_max_index[i] = right_max_index[i + 1];
        }
    }

    for i in k..=(n - 2 * k) {
        let left_index = left_max_index[i - 1];
        let right_index = right_max_index[i + k];
        let total_sum = (prefix_sum[i + k] - prefix_sum[i])
            + (prefix_sum[left_index + k] - prefix_sum[left_index])
            + (prefix_sum[right_index + k] - prefix_sum[right_index]);

        if total_sum > max_sum {
            max_sum = total_sum;
            result[0] = left_index as i32;
            result[1] = i as i32;
            result[2] = right_index as i32;
        }
    }

    result
}
