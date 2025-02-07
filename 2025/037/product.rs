// https://leetcode.com/problems/tuple-with-same-product/description/

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::<i32, i32>::new();
    let n = nums.len();

    for i in 0..n {
        for j in (i + 1)..n {
            *freq.entry(nums[i] * nums[j]).or_default() += 1;
        }
    }

    freq.into_iter()
        .map(|(val, freq)| if freq > 1 { 4 * freq * (freq - 1) } else { 0 })
        .sum()
}
