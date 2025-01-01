// https://leetcode.com/problems/maximum-width-ramp/description/

pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right_max = vec![0; n];
    right_max[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(nums[i]);
    }

    let mut left = 0;
    let mut right = 0;
    let mut max_width = 0;

    while right < n {
        while left < right && nums[left] > right_max[right] {
            left += 1;
        }
        max_width = max_width.max(right - left);
        right += 1;
    }

    max_width as i32
}
