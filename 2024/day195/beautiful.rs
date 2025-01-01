// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/

pub fn min_changes(s: String) -> i32 {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .filter(|x| x[0] != x[1])
        .count() as i32
}
