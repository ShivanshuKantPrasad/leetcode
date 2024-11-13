// https://leetcode.com/problems/count-the-number-of-fair-pairs/description/

pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();

    fn lower_bound(nums: &[i32], value: i32) -> i64 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = 0;
        while left < right {
            let sum = nums[left] + nums[right];
            if sum < value {
                result += (right - left) as i64;
                left += 1;
            } else {
                right -= 1;
            }
        }
        result
    }

    lower_bound(&nums, upper + 1) - lower_bound(&nums, lower)
}
