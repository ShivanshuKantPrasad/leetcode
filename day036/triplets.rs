// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut prefix = 0;

        let mut count_map = HashMap::<i32, i32>::new();
        count_map.insert(0, 1);
        let mut total_map = HashMap::<i32, i32>::new();

        for i in 0..arr.len() {
            prefix ^= arr[i];
            count += *count_map.entry(prefix).or_default() * i as i32
                - *total_map.entry(prefix).or_default();
            *count_map.entry(prefix).or_default() += 1;
            *total_map.entry(prefix).or_default() += i as i32 + 1;
        }

        count
    }
}

fn main() {
    println!("{}", Solution::count_triplets(vec![2, 3, 1, 6, 7]));
    println!("{}", Solution::count_triplets(vec![1, 1, 1, 1, 1]));
}
