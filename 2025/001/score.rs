// https://leetcode.com/problems/maximum-score-after-splitting-a-string/description/

pub fn max_score(s: String) -> i32 {
    let mut ones = s.chars().filter(|c| *c == '1').count() as i32;
    let s = s.as_bytes();

    let mut ans = 0;
    let mut zeros = 0;

    for i in 0..(s.len() - 1) {
        if s[i] == b'1' {
            ones -= 1;
        } else {
            zeros += 1;
        }
        ans = ans.max(zeros + ones);
    }

    ans
}
