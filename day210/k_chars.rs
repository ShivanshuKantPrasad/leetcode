// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/description/

pub fn take_characters(s: String, k: i32) -> i32 {
    let mut count = vec![0; 3];
    let n = s.len();

    for ch in s.bytes() {
        count[(ch - b'a') as usize] += 1;
    }

    for i in 0..3 {
        if count[i] < k {
            return -1;
        }
    }

    let mut window = vec![0; 3];
    let mut left = 0;
    let mut max = 0;

    let s = s.as_bytes();

    for right in 0..n {
        window[(s[right] - b'a') as usize] += 1;

        while left <= right && (0..3).any(|i| count[i] - window[i] < k) {
            window[(s[left] - b'a') as usize] -= 1;
            left += 1;
        }

        max = max.max(right - left + 1);
    }

    (n - max) as i32
}
