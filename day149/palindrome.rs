// https://leetcode.com/problems/shortest-palindrome/

pub fn shortest_palindrome(s: String) -> String {
    let n = s.len();
    let mut reversed_string = s.chars().rev().collect::<String>();

    for i in 0..n {
        if s[..n - i] == reversed_string[i..] {
            return reversed_string[..i].to_string() + &s;
        }
    }

    String::new()
}
