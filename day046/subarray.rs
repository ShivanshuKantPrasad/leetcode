// https://leetcode.com/problems/subarray-sums-divisible-by-k/submissions/1282392681/

struct Solution;
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = vec![0; k as usize];
        freq[0] = 1;

        let mut sum = 0;
        let mut result = 0;

        for num in nums {
            sum = (sum + num).rem_euclid(k);
            result += freq[sum as usize];
            freq[sum as usize] += 1;
        }

        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5)
    );
    println!("{}", Solution::subarrays_div_by_k(vec![5], 9));
}
