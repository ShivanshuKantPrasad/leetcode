// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/

pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let M = grid.len();
    let N = grid[0].len();

    let mut dp = vec![vec![0; 2]; M];

    for i in 0..M {
        dp[i][0] = 1;
    }

    let mut max = 0;
    for j in 1..N {
        for i in 0..M {
            if grid[i][j] > grid[i][j - 1] && dp[i][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i][0] + 1);
            }

            if i > 0 && grid[i][j] > grid[i - 1][j - 1] && dp[i - 1][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i - 1][0] + 1);
            }

            if i + 1 < M && grid[i][j] > grid[i + 1][j - 1] && dp[i + 1][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i + 1][0] + 1);
            }

            max = max.max(dp[i][1] - 1);
        }

        for k in 0..M {
            dp[k][0] = dp[k][1];
            dp[k][1] = 0;
        }
    }

    max
}
