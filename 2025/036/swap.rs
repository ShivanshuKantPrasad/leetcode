// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/description/

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut num_diff = 0;
    s1.bytes()
        .zip(s2.bytes())
        .fold([0i32; 26], |mut acc, (ch1, ch2)| {
            if ch1 != ch2 {
                num_diff += 1;
            }
            acc[(ch1 - b'a') as usize] += 1;
            acc[(ch2 - b'a') as usize] -= 1;
            acc
        })
        .iter()
        .all(|&c| c == 0)
        && num_diff <= 2
}
