// https://leetcode.com/problems/clear-digits/description/

pub fn clear_digits(s: String) -> String {
    let mut result = Vec::with_capacity(s.len());
    let s = s.chars();

    for ch in s {
        if ch.is_numeric() {
            result.pop();
        } else {
            result.push(ch);
        }
    }

    result.iter().collect()
}
