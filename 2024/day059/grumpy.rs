// https://leetcode.com/problems/grumpy-bookstore-owner/

struct Solution;
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let max: i32 = customers
            .iter()
            .enumerate()
            .map(|(i, x)| (1 - grumpy[i]) * x)
            .sum();

        let y = customers
            .iter()
            .enumerate()
            .map(|(i, x)| grumpy[i] * x)
            .collect::<Vec<_>>();

        let x: i32 = y
            .windows(minutes as usize)
            .map(|x| x.iter().sum())
            .max()
            .unwrap();
        // .unwrap_or(y.iter().sum());

        max + x
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        )
    );
    println!("{}", Solution::max_satisfied(vec![1], vec![0], 1));
    println!("{}", Solution::max_satisfied(vec![3], vec![1], 1));
}
