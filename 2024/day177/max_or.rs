// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/

pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or = nums.iter().fold(0, |acc, num| acc | num);
    let total_subsets = 1 << nums.len();
    (0..total_subsets).fold(0, |res, mask| {
        let cur_or = nums.iter().enumerate().fold(0, |acc, (index, num)| {
            if ((mask >> index) & 1) == 1 {
                acc | num
            } else {
                acc
            }
        });

        if cur_or == max_or {
            res + 1
        } else {
            res
        }
    })
}
