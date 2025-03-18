// https://leetcode.com/problems/longest-nice-subarray/description/

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut used_bits = 0;
    let mut window_start = 0;
    let mut max_length = 0;

    for window_end in 0..nums.len() {
        while used_bits & nums[window_end] != 0 {
            used_bits ^= nums[window_start];
            window_start += 1;
        }

        used_bits |= nums[window_end];
        max_length = max_length.max(window_end - window_start + 1);
    }

    max_length as i32
}
