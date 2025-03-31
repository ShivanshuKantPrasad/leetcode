// https://leetcode.com/problems/put-marbles-in-bags/submissions/1592182419/

pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let n = weights.len();
    let mut pair_weights = vec![0; n - 1];
    for i in 0..(n - 1) {
        pair_weights[i] += weights[i] + weights[i + 1];
    }

    pair_weights.sort_unstable();

    let mut answer = 0;
    for i in 0..(k as usize - 1) {
        answer += (pair_weights[n - 2 - i] - pair_weights[i]) as i64;
    }

    answer
}
