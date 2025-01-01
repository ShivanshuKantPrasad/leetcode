// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let row = matrix.len();
    let col = matrix[0].len();
    let mut dp = vec![vec![0; col + 1]; row + 1];

    let mut ans = 0;

    for i in 0..row {
        for j in 0..col {
            if (matrix[i][j] == 1) {
                dp[i + 1][j + 1] = [dp[i][j + 1], dp[i + 1][j], dp[i][j]].iter().min().unwrap() + 1;
                ans += dp[i + 1][j + 1];
            }
        }
    }

    ans
}
