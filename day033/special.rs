// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/submissions/1268986629/

struct Solution;
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 1001];

        for num in nums {
            for i in (0..(num + 1) as usize).rev() {
                freq[i] += 1;
            }
        }

        for i in 0..freq.len() {
            if i == freq[i] {
                return i as i32;
            }
        }

        return -1;
    }
}

fn main() {
    println!("{}", Solution::special_array(vec![3, 5]));
    println!("{}", Solution::special_array(vec![0, 0]));
    println!("{}", Solution::special_array(vec![0, 4, 3, 0, 4]));
}
