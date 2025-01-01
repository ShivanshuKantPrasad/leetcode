// https://leetcode.com/problems/maximum-matrix-sum/

pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let neg_count = matrix.iter().flatten().filter(|val| **val < 0).count();

    let mat = matrix.iter().flatten().map(|x| x.abs() as i64);
    let total_sum: i64 = mat.clone().sum();
    let min = mat.min().unwrap();

    total_sum - if neg_count % 2 == 0 { 0 } else { 2 * min.abs() }
}
