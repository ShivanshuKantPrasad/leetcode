// https://leetcode.com/problems/score-after-flipping-matrix/description/

struct Solution;
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut grid = grid;
        let mut ones = vec![0; n];
        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n {
                    grid[i][j] = 1 - grid[i][j];
                    if grid[i][j] == 1 {
                        ones[j] += 1;
                    }
                }
            }
        }

        for j in 0..n {
            let mut countOnes = 0;

            for i in 0..m {
                countOnes += grid[i][j];
            }
            let countZeros = m as i32 - countOnes;
            if countZeros > countOnes {
                for i in 0..m {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }

        println!("{grid:?}");

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                res += grid[i][j] << (n - j - 1);
            }
        }
        res
    }
}

fn main() {
    println!(
        "{}",
        Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]])
    );
    println!("{}", Solution::matrix_score(vec![vec![0]]));
    println!(
        "{}",
        Solution::matrix_score(vec![vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 0]])
    );
}
