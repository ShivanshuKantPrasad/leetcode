// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/submissions/1386051733/

pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
    (0..32).fold(0, |res, i| {
        res + 1 - ((start >> i) & 1 == (goal >> i) & 1) as i32
    })
}
