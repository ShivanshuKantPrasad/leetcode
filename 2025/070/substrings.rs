// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/description/

pub fn number_of_substrings(s: String) -> i32 {
    let n = s.len();
    let mut last_pos = vec![-1, -1, -1];
    let mut total = 0;

    let s = s.as_bytes();
    for pos in 0..n {
        last_pos[(s[pos] - b'a') as usize] = pos as i32;
        total += 1 + *last_pos.iter().min().unwrap();
    }
    total
}
