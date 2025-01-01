// https://leetcode.com/problems/minimum-falling-path-sum-ii/

struct Solution;
impl Solution {
    pub fn optimal(row: usize, grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) {
        if row == grid.len() - 1 {
            for i in 0..grid[row].len() {
                memo[row][i] = grid[row][i];
            }
        } else {
            Self::optimal(row + 1, grid, memo);
            for i in 0..grid[row].len() {
                let min = *memo[row + 1]
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, x)| x)
                    .min()
                    .unwrap();
                memo[row][i] += min + grid[row][i];
            }
        }
    }

    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = vec![vec![0; grid.len()]; grid.len()];
        Self::optimal(0, &grid, &mut memo);
        println!("{memo:?}");
        *memo[0].iter().min().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!("{}", Solution::min_falling_path_sum(vec![vec![7]]));
}
