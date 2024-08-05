// https://leetcode.com/problems/kth-distinct-string-in-an-array/description/

pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
    use std::collections::HashMap;
    let mut freq = HashMap::<String, i32>::new();
    for s in arr.clone() {
        *freq.entry(s).or_default() += 1;
    }

    for s in arr {
        if *freq.get(&s).unwrap() == 1 {
            k -= 1;
        }
        if k == 0 {
            return s;
        }
    }

    String::new()
}
