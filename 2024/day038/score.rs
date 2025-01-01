// https://leetcode.com/problems/score-of-a-string/

struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .fold(0, |a, c| a + i32::abs(c[0] as i32 - c[1] as i32))
    }
}

fn main() {
    println!("{}", Solution::score_of_string("hello".to_string()));
    println!("{}", Solution::score_of_string("zaz".to_string()));
}
