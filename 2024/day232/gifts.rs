// https://leetcode.com/problems/take-gifts-from-the-richest-pile/description/

pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut gifts = BinaryHeap::from_iter(gifts.iter().map(|x| *x as i64));

    for _ in 0..k {
        match gifts.pop() {
            Some(x) => {
                gifts.push((x as f64).sqrt().floor() as i64);
            }
            None => {}
        }
    }

    gifts.iter().sum()
}
