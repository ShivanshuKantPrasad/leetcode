// https://leetcode.com/problems/convert-1d-array-into-2d-array/submissions/1375672964/

pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if m * n != original.len() as i32 {
        return vec![];
    };

    let m = m as usize;
    let n = n as usize;
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            result[i][j] = original[i * n + j];
        }
    }
    result
}
