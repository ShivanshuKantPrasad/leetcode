// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/

pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = row_sum.len();
    let m = col_sum.len();
    let mut orig_matrix = vec![vec![0; m]; n];

    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        orig_matrix[i][j] = i32::min(row_sum[i], col_sum[j]);
        row_sum[i] -= orig_matrix[i][j];
        col_sum[j] -= orig_matrix[i][j];

        if row_sum[i] == 0 {
            i += 1;
        } else {
            j += 1;
        }
    }
    orig_matrix
}
