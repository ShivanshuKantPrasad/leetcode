// https://leetcode.com/problems/minimum-length-of-string-after-operations/description/

pub fn minimum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let mut freqs = vec![0; 26];

    for c in s {
        freqs[(c - b'a') as usize] += 1;
    }

    let mut result = 0;
    for freq in freqs {
        if freq < 3 {
            result += freq;
        } else {
            result += (freq + 1) % 2 + 1;
        }
    }
    result
}

// 3 -> 1
// 4 -> 2
// 5 -> 3 -> 1
