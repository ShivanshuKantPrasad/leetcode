// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/

pub fn find_the_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let bitmasks = HashMap::from([('a', 1), ('e', 2), ('i', 4), ('o', 8), ('u', 16)]);

    let mut prefix = vec![0; s.len() + 1];

    for (i, ch) in s.char_indices() {
        prefix[i + 1] = prefix[i] ^ *bitmasks.get(&ch).unwrap_or(&0);
    }

    let mut res = 0;
    for i in 0..prefix.len() {
        for j in (i..prefix.len()).rev() {
            if prefix[i] ^ prefix[j] == 0 {
                res = res.max(j - i);
                break;
            }
        }
    }

    res as i32
}
