// https://leetcode.com/problems/magnetic-force-between-two-balls/description/

pub fn place_with_gap(position: &Vec<i32>, mut m: i32, gap: i32) -> bool {
    let mut prev = position[0];
    m -= 1;
    for i in 1..position.len() {
        if position[i] - prev >= gap {
            prev = position[i];
            m -= 1;
        }
    }
    if m <= 0 {
        true
    } else {
        false
    }
}
pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort_unstable();
    let mut lo = 1;
    let mut hi = (position.last().unwrap() - position[0]) / (m - 1);

    while lo < hi {
        let mid = (hi + lo) / 2 + 1;
        if place_with_gap(&position, m, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    lo
}

fn main() {
    println!("{}", max_distance(vec![1, 2, 3, 4, 7], 3));
    println!("{}", max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2));
    println!("{}", max_distance(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4));
}
