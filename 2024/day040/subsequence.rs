// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/editorial/

struct Solution;
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while j != t.len() && i != s.len() {
            if s[i] == t[j] {
                j += 1;
            }
            i += 1;
        }
        return (t.len() - j) as i32;
    }
}

fn main() {
    println!(
        "{}",
        Solution::append_characters("coaching".to_string(), "coding".to_string())
    );
    println!(
        "{}",
        Solution::append_characters("abcde".to_string(), "a".to_string())
    );
    println!(
        "{}",
        Solution::append_characters("z".to_string(), "abcde".to_string())
    );
}
