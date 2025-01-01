// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut prefixes = HashSet::new();

    for mut num in arr1 {
        while num != 0 {
            prefixes.insert(num);
            num /= 10;
        }
    }

    let mut res = 0;
    for mut num in arr2 {
        while num != 0 {
            if prefixes.contains(&num) {
                res = res.max(num.ilog10() + 1);
                break;
            }
            num /= 10;
        }
    }

    res as i32
}
