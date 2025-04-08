// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/description/

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut members = HashSet::new();

    let n = nums.len();
    for (i, num) in nums.into_iter().enumerate().rev() {
        if !members.contains(&num) {
            members.insert(num);
        } else {
            return (i + 1).div_ceil(3) as i32;
        }
    }

    0
}
