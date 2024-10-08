// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/

pub fn min_swaps(s: String) -> i32 {
    (s.chars().fold(0, |acc, ch| {
        if ch == '[' {
            acc + 1
        } else {
            if acc > 0 {
                acc - 1
            } else {
                acc
            }
        }
    }) + 1)
        / 2
}
