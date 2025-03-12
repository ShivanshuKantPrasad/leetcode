// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/description/
use std::cmp::Ordering::*;

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut neg = 0;
    match nums.binary_search_by(|probe| probe.cmp(&-1).then(Less)) {
        Err(id) => neg = id as i32,
        _ => unreachable!(),
    }

    let mut pos = 0;
    let pos_id = nums.binary_search_by(|probe| probe.cmp(&1).then(Greater));

    match pos_id {
        Err(id) => pos = (nums.len() - id) as i32,
        _ => unreachable!(),
    }

    neg.max(pos)
}
