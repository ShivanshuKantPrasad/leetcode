// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/description/

pub fn min_length(mut s: String) -> i32 {
    while s.contains("AB") || s.contains("CD") {
        s = s.replace("AB", "");
        s = s.replace("CD", "");
    }
    s.len() as i32
}
