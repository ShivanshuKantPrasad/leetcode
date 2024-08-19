// https://leetcode.com/problems/2-keys-keyboard/submissions/1360947185/

pub fn min_steps(mut n: i32) -> i32 {
    let mut ans = 0;
    let mut d = 2;

    while n > 1 {
        while n % d == 0 {
            ans += d;
            n /= d;
        }
        d += 1;
    }
    ans
}
