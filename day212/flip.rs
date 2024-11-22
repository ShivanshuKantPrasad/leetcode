// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/description/

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut pattern_freq = HashMap::new();

    for current_row in matrix.iter() {
        let mut pattern_builder = String::new();
        for col in 0..current_row.len() {
            pattern_builder += if current_row[0] == current_row[col] {
                "T"
            } else {
                "F"
            }
        }
        *pattern_freq.entry(pattern_builder).or_insert(0) += 1;
    }

    let mut max_freq = 0;
    pattern_freq.into_iter().map(|x| x.1).max().unwrap()
}
