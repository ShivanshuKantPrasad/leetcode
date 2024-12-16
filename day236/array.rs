// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/description/

pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = nums.len();

    let mut nums: BinaryHeap<_> = nums
        .into_iter()
        .enumerate()
        .map(|(i, x)| Reverse((x, i)))
        .collect();

    for _ in 0..k {
        match nums.pop() {
            Some(Reverse((x, i))) => {
                nums.push(Reverse((x * multiplier, i)));
            }
            None => {}
        }
    }

    let mut res = vec![0; n];
    while let Some(Reverse((x, i))) = nums.pop() {
        res[i] = x;
    }
    res
}
