// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/description/

pub fn minimum_deletions(s: String) -> i32 {
    let s = s.as_bytes();
    let mut min_deletions = 0;
    let mut b_count = 0;
    for i in 0..s.len() {
        if s[i] == b'b' {
            b_count += 1;
        } else {
            min_deletions = b_count.min(min_deletions + 1);
        }
    }

    return min_deletions;
}
