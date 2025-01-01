// https://leetcode.com/problems/get-equal-substrings-within-budget/description/

struct Solution;
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut cost = vec![0; s.len()];

        for i in 0..s.len() {
            cost[i] = i32::abs(s[i] as i32 - t[i] as i32);
        }

        let mut max_len = 0;
        let mut cur_cost = 0;

        let mut i = 0;
        let mut j = 0;

        while i < s.len() && j < s.len() {
            cur_cost += cost[j];
            if cur_cost <= max_cost {
                max_len = max_len.max(j - i + 1);
            } else {
                cur_cost -= cost[i];
                i += 1;
            }
            j += 1;
        }
        max_len as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3)
    );
    println!(
        "{}",
        Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3)
    );
    println!(
        "{}",
        Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0)
    );
}
