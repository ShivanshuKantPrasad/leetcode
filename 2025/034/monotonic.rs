// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/description/

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    let mut increasing = false;
    let mut decreasing = false;
    let mut cur_len = 1;
    let n = nums.len();

    for i in 1..n {
        if nums[i] > nums[i - 1] {
            if increasing {
                cur_len += 1;
            } else {
                cur_len = 2;
                decreasing = false;
                increasing = true;
            }
        } else if nums[i] < nums[i - 1] {
            if decreasing {
                cur_len += 1;
            } else {
                cur_len = 2;
                decreasing = true;
                increasing = false;
            }
        } else {
            cur_len = 1;
            increasing = false;
            decreasing = false;
        }
        res = res.max(cur_len);
    }

    res
}
