// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/description/

pub fn find_kth_bit(n: i32, k: i32) -> char {
    let mut s = "0".to_string();
    let k = k as usize - 1;

    for _ in 0..n {
        s = format!(
            "{s}1{}",
            s.chars()
                .map(|ch| if ch == '1' { '0' } else { '1' })
                .rev()
                .collect::<String>()
        );
        if s.len() > k {
            return s.chars().nth(k).unwrap();
        }
    }
    s.chars().nth(k).unwrap()
}
