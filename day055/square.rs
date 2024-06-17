// https://leetcode.com/problems/sum-of-square-numbers/

struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let n = (c as f64).sqrt() as i32;

        for i in 0..=n {
            let x = c - (i * i);
            let xsqrt = (x as f64).sqrt() as i32;
            if xsqrt * xsqrt == x {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("{}", Solution::judge_square_sum(5));
    println!("{}", Solution::judge_square_sum(3));
    println!("{}", Solution::judge_square_sum(2));
}
