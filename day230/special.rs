// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/submissions/1475290037/

pub fn maximum_length(s: String) -> i32 {
    let s = s.as_bytes();

    use std::collections::HashMap;
    let mut count = HashMap::<(u8, i32), i32>::new();

    let mut sub_len = 0;

    for start in 0..s.len() {
        let ch = s[start];
        sub_len = 0;
        for end in start..s.len() {
            if ch == s[end] {
                sub_len += 1;
                *count.entry((ch, sub_len)).or_insert(0) += 1;
            } else {
                break;
            }
        }
    }

    let mut ans = 0;
    for i in count {
        let len = i.0 .1;
        if i.1 >= 3 && len > ans {
            ans = len;
        }
    }
    if ans == 0 {
        -1
    } else {
        ans
    }
}
