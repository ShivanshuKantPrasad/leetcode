// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 3 {
        return 0;
    }
    nums.sort_unstable();
    let mut result = i32::MAX;
    for i in 0..4 {
        result = result.min(nums[len - i - 1] - nums[3 - i]);
    }
    result
}

fn main() {
    println!("{}", min_difference(vec![5, 3, 2, 4]));
    println!("{}", min_difference(vec![1, 5, 0, 10, 14]));
    println!("{}", min_difference(vec![3, 100, 20]));
    println!("{}", min_difference(vec![6, 6, 0, 1, 1, 4, 6]));
}
