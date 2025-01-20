pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut indices = vec![(0, 0); arr.len() + 1];

    let m = mat.len();
    let n = mat[0].len();

    for i in 0..m {
        for j in 0..n {
            indices[mat[i][j] as usize] = (i, j);
        }
    }

    let mut row_paint_freq = vec![0; m];
    let mut col_paint_freq = vec![0; n];

    for (index, &num) in arr.iter().enumerate() {
        let (row, col) = indices[num as usize];
        row_paint_freq[row] += 1;
        col_paint_freq[col] += 1;
        if row_paint_freq[row] == n || col_paint_freq[col] == m {
            return index as i32;
        }
    }

    unreachable!();
}
