// https://leetcode.com/problems/most-profit-assigning-work/

struct Solution;
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let ns = *worker.iter().max().unwrap() as usize;
        let mut max_profit = vec![0; ns + 1];

        for i in 0..profit.len() {
            let d = difficulty[i] as usize;
            if d <= ns {
                max_profit[d] = max_profit[d].max(profit[i]);
            }
        }

        for x in 1..max_profit.len() {
            max_profit[x] = max_profit[x].max(max_profit[x - 1]);
        }

        worker.into_iter().map(|w| max_profit[w as usize]).sum()
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_profit_assignment(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7]
        )
    );
    println!(
        "{}",
        Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25])
    );
    println!(
        "{}",
        Solution::max_profit_assignment(vec![13, 37, 58], vec![4, 90, 96], vec![34, 73, 45])
    );
}
