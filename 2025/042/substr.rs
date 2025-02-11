// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/description/

pub fn remove_occurrences(mut s: String, part: String) -> String {
    while let Some(index) = s.find(&part) {
        s = s[..index].to_string() + &s[(index + part.len())..];
    }
    s
}
