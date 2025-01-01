// https://leetcode.com/problems/lucky-numbers-in-a-matrix/description/

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut row_min = vec![i32::MAX; m];
    let mut column_max = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            row_min[i] = row_min[i].min(matrix[i][j]);
            column_max[j] = column_max[j].max(matrix[i][j]);
        }
    }

    let mut result = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if row_min[i] == matrix[i][j] && column_max[j] == matrix[i][j] {
                result.push(matrix[i][j]);
            }
        }
    }
    result
}
