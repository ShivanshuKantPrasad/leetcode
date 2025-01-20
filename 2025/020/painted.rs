// https://leetcode.com/problems/first-completely-painted-row-or-column/description/

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut indices = HashMap::new();

    let m = mat.len();
    let n = mat[0].len();

    for i in 0..m {
        for j in 0..n {
            indices.insert(mat[i][j], (i, j));
        }
    }

    let mut paint = vec![vec![false; n]; m];

    fn is_painted(
        paint: &Vec<Vec<bool>>,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) -> bool {
        let mut condition = true;
        for i in 0..width {
            if paint[row][i] == false {
                condition = false;
                break;
            }
        }
        if condition == true {
            return true;
        }

        condition = true;
        for i in 0..height {
            if paint[i][col] == false {
                return false;
            }
        }

        condition
    }

    for (index, num) in arr.iter().enumerate() {
        let (row, col) = indices[num];
        paint[row][col] = true;

        if is_painted(&paint, row, col, m, n) {
            return index as i32;
        }
    }

    0
}
