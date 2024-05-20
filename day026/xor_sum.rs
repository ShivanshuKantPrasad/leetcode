// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/
//
struct Solution;
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut res = 0;
        loop {
            let mut xor = 0;
            for j in 0..nums.len() {
                xor = xor ^ nums[j] * ((i >> j) & 1);
            }
            res += xor;
            i += 1;
            if i == i32::pow(2, nums.len() as u32) {
                break;
            }
        }
        res
    }
}

fn main() {
    println!("{}", Solution::subset_xor_sum(vec![1, 3]));
    println!("{}", Solution::subset_xor_sum(vec![5, 1, 6]));
    println!("{}", Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
}
