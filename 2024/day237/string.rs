// https://leetcode.com/problems/construct-string-with-repeat-limit/description/

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut freq = vec![0; 26];
    for ch in s.as_bytes() {
        freq[(ch - b'a') as usize] += 1;
    }

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();

    for (i, &count) in freq.iter().enumerate() {
        if count != 0 {
            heap.push(i);
        }
    }

    let mut result = String::new();

    while let Some(ch) = heap.pop() {
        let count = freq[ch];

        let used = count.min(repeat_limit);
        for _ in 0..used {
            result.push((b'a' + ch as u8) as char);
        }

        freq[ch] -= used;
        if freq[ch] > 0 && !heap.is_empty() {
            let next = heap.pop().unwrap();
            result.push((b'a' + next as u8) as char);
            freq[next] -= 1;

            if freq[next] > 0 {
                heap.push(next);
            }
            heap.push(ch);
        }
    }

    result
}
