// https://leetcode.com/problems/single-number-iii/description/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = HashMap::<i32, i32>::new();

        for num in nums {
            *freq.entry(num).or_default() += 1;
        }

        freq.iter()
            .filter(|(_, frequency)| **frequency == 1)
            .map(|(num, _)| *num)
            .collect::<Vec<i32>>()
    }
}

fn main() {
    println!("{:?}", Solution::single_number(vec![1, 2, 1, 3, 2, 5]));
    println!("{:?}", Solution::single_number(vec![-1, 0]));
    println!("{:?}", Solution::single_number(vec![0, 1]));
}
