// https://leetcode.com/problems/make-sum-divisible-by-p/description/

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let n = nums.len() as i32;

    let target = nums.iter().fold(0, |acc, x| (acc + x) % p);
    if target == 0 {
        return 0;
    }

    use std::collections::HashMap;
    let mut mod_map = HashMap::new();
    mod_map.insert(0, -1);

    let mut current_sum = 0;
    let mut min_len = n;

    for i in 0..n {
        current_sum = (current_sum + nums[i as usize]) % p;

        let mut needed = (current_sum - target + p) % p;

        if let Some(val) = mod_map.get(&needed) {
            min_len = min_len.min(i - val);
        }

        mod_map.insert(current_sum, i);
    }

    if min_len == n {
        -1
    } else {
        min_len
    }
}
