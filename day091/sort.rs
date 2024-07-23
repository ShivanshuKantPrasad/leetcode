// https://leetcode.com/problems/sort-array-by-increasing-frequency/

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut freq = HashMap::<i32, i32>::new();
    for &num in &nums {
        *freq.entry(num).or_default() += 1;
    }
    let mut nums_with_freq = nums
        .iter()
        .map(|num| (*num, freq.get(num).unwrap()))
        .collect::<Vec<_>>();
    nums_with_freq.sort_unstable_by(|num1, num2| {
        if num1.1 == num2.1 {
            num2.0.cmp(&num1.0)
        } else {
            num1.1.cmp(&num2.1)
        }
    });
    nums_with_freq.into_iter().map(|num| num.0).collect()
}
