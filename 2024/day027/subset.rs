// https://leetcode.com/problems/subsets/

struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![vec![], nums];
        }
        let small = Self::subsets(nums[1..].to_vec());
        let mut res = Vec::<Vec<i32>>::new();
        res.extend(small.clone());
        for mut x in small {
            x.push(nums[0]);
            res.push(x);
        }
        return res;
    }
}

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
    println!("{:?}", Solution::subsets(vec![0]));
}
