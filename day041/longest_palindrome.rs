// https://leetcode.com/problems/longest-palindrome/

struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq = vec![0; 256];
        for c in s.as_bytes() {
            freq[*c as usize] += 1;
        }

        let mut result = 0;
        let mut carry = 0;
        for i in freq {
            result += 2 * (i / 2);
            if i % 2 == 1 {
                carry = 1;
            }
        }
        result + carry
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("abccccdd".to_string()));
    println!("{}", Solution::longest_palindrome("a".to_string()));
}
