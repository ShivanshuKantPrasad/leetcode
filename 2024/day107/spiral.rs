// https://leetcode.com/problems/spiral-matrix-iii/

pub fn spiral_matrix_iii(
    rows: i32,
    cols: i32,
    mut r_start: i32,
    mut c_start: i32,
) -> Vec<Vec<i32>> {
    let mut length = 1;
    let max_length = 2 * rows.max(cols);

    let mut res = Vec::new();

    if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
        res.push(vec![r_start, c_start]);
    }

    println!("{}", max_length);
    while length <= max_length {
        for i in 0..length {
            c_start += 1;
            if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
                res.push(vec![r_start, c_start]);
            }
        }
        for i in 0..length {
            r_start += 1;
            if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
                res.push(vec![r_start, c_start]);
            }
        }
        length += 1;
        for i in 0..length {
            c_start -= 1;
            if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
                res.push(vec![r_start, c_start]);
            }
        }
        for i in 0..length {
            r_start -= 1;
            if 0 <= r_start && r_start < rows && 0 <= c_start && c_start < cols {
                res.push(vec![r_start, c_start]);
            }
        }
        length += 1;
    }

    res
}
