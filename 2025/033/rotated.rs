// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/description/

pub fn check(nums: Vec<i32>) -> bool {
    let mut rotated = false;
    let mut sorted = true;
    let n = nums.len();

    for i in 0..nums.len() {
        if nums[(i + 1) % n] >= nums[i] {
        } else {
            if !rotated {
                rotated = true;
            } else {
                sorted = false;
            }
        }
    }
    sorted
}
