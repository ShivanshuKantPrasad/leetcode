// https://leetcode.com/problems/maximize-happiness-of-selected-children/description/

struct Solution;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_unstable_by_key(|&x| -x);
        happiness
            .iter()
            .enumerate()
            .map(|(i, x)| (x - i as i32).max(0) as i64)
            .take(k as usize)
            .sum()
    }
}

fn main() {
    println!("{}", Solution::maximum_happiness_sum(vec![1, 2, 3], 2));
    println!("{}", Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2));
    println!("{}", Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1));
}
