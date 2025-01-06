// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/

pub fn min_operations(boxes: String) -> Vec<i32> {
    let boxes = boxes.as_bytes();
    let mut result = vec![0; boxes.len()];

    for i in 0..boxes.len() {
        if boxes[i] == b'1' {
            for j in 0..boxes.len() {
                result[j] += (j as i32 - i as i32).abs();
            }
        }
    }

    result
}
