//https://leetcode.com/problems/count-number-of-bad-pairs/

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let mut counter = HashMap::<i32, i64>::new();
    let mut result = 0;

    for (index, num) in nums.iter().enumerate() {
        let index = index as i32;
        let mut diff_count = counter.entry(num - index).or_default();
        result += index as i64 - *diff_count;
        *diff_count += 1;
    }

    result
}
