// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/description/

use std::collections::VecDeque;

pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = vec![0; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        prefix[i + 1] = prefix[i] + num as i64;
    }

    let mut candidate_indices = VecDeque::new();

    let mut min = usize::MAX;

    for i in 0..=nums.len() {
        while !candidate_indices.is_empty()
            && prefix[i] - prefix[*candidate_indices.front().unwrap()] >= k as i64
        {
            min = min.min(i - candidate_indices.front().unwrap());
            candidate_indices.pop_front();
        }

        while !candidate_indices.is_empty()
            && prefix[i] <= prefix[*candidate_indices.back().unwrap()]
        {
            candidate_indices.pop_back();
        }

        candidate_indices.push_back(i);
    }

    if min == usize::MAX {
        -1
    } else {
        min as i32
    }
}
