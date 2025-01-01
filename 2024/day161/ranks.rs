// https://leetcode.com/problems/rank-transform-of-an-array/

pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut sort = arr.clone();
    sort.sort_unstable();
    sort.dedup();

    let rank: HashMap<_, _> = sort
        .iter()
        .enumerate()
        .map(|(idx, num)| (*num, idx as i32 + 1))
        .collect();
    arr.iter().map(|x| *rank.get(x).unwrap()).collect()
}
