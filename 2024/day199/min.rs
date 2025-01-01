// https://leetcode.com/problems/minimum-array-end/description/

pub fn min_end(n: i32, x: i32) -> i64 {
    let x = x as i64;
    (0..(n - 1)).fold(x, |res, _| (res + 1) | x)
}
