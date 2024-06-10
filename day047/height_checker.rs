//https://leetcode.com/problems/height-checker/description/

struct Solution;
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort_unstable();

        std::iter::zip(heights, sorted)
            .filter(|(x, y)| *x != *y)
            .count() as i32
    }
}

fn main() {
    println!("{}", Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
    println!("{}", Solution::height_checker(vec![5, 1, 2, 3, 4]));
    println!("{}", Solution::height_checker(vec![1, 2, 3, 4, 5]));
}
