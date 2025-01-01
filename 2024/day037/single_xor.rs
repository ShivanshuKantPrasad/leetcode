// https://leetcode.com/problems/single-number-iii/description/

struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, num| acc ^ num);

        let lowest_bit = xor & -xor;

        let mut result = vec![0, 0];
        for num in nums {
            if lowest_bit & num == 0 {
                result[0] ^= num;
            } else {
                result[1] ^= num;
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
    println!("{:?}", Solution::single_number(vec![-1, 0]));
    println!("{:?}", Solution::single_number(vec![0, 1]));
}
