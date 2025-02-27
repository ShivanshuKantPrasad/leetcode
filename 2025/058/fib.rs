// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/description/
use std::collections::{HashMap, HashSet};

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let mut dp = HashMap::new();
    let set: HashSet<i32> = arr.iter().cloned().collect();
    let mut max_len = 0;

    for i in 0..arr.len() {
        for j in 0..i {
            let x = arr[j];
            let y = arr[i];
            let z = y - x;

            if z < x && set.contains(&z) {
                let len = *dp.get(&(z, x)).unwrap_or(&2) + 1;
                dp.insert((x, y), len);
                max_len = max_len.max(len);
            }
        }
    }

    max_len
}
