// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/submissions/1445822505/

pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut bits = [0; 24];

    for num in candidates {
        for i in 0..24 {
            if (1 << i) & num != 0 {
                bits[i] += 1;
            }
        }
    }

    bits.into_iter().max().unwrap_or(0)
}
