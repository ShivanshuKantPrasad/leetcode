// https://leetcode.com/problems/patching-array/

struct Solution;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0i64;

        let mut i = 0;
        while i < nums.len() && sum < n as i64 {
            if nums[i] as i64 <= sum + 1 {
                sum += nums[i] as i64;
                i += 1;
            } else {
                ans += 1;
                sum = sum * 2 + 1;
            }
        }

        while sum < n as i64 {
            sum = sum * 2 + 1;
            ans += 1;
        }

        return ans;
    }
}

fn main() {
    println!("{}", Solution::min_patches(vec![1, 3], 6));
    println!("{}", Solution::min_patches(vec![1, 5, 10], 20));
    println!("{}", Solution::min_patches(vec![1, 2, 2], 5));
    println!("{}", Solution::min_patches(vec![1, 2, 31, 33], 2147483647));
}
