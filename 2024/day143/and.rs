// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max = nums.iter().max().unwrap();

    let mut res = 0;

    let mut i = 0;
    while i < nums.len() {
        let mut j = i;
        while j < nums.len() && nums[j] == *max {
            j += 1;
        }
        res = res.max(j - i);
        i = if i == j { i + 1 } else { j };
    }
    res as i32
}
