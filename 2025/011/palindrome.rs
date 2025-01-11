// https://leetcode.com/problems/construct-k-palindrome-strings/description/

pub fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;

    let mut count = [0; 26];
    for &b in s.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }

    if s.len() < k || count.into_iter().filter(|x| x % 2 == 1).count() > k {
        false
    } else {
        true
    }
}
