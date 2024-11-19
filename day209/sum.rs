// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/submissions/1457290690/

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;

    let k = k as usize;
    let mut sum: i64 = nums.iter().take(k).map(|x| *x as i64).sum();

    let mut freq = HashMap::new();
    for &num in &nums[0..k] {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut max = (if freq.len() == k { sum } else { 0 });

    for i in 0..(nums.len() - k) {
        sum = sum - nums[i] as i64 + nums[i + k] as i64;
        *freq.entry(nums[i]).or_insert(0) -= 1;
        if freq.get(&nums[i]) == Some(&0) {
            freq.remove(&nums[i]);
        }
        *freq.entry(nums[i + k]).or_insert(0) += 1;

        if freq.len() == k {
            max = max.max(sum);
        }
    }

    max
}
