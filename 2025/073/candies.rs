// https://leetcode.com/problems/maximum-candies-allocated-to-k-children/description/

pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    fn allocate_candies(candies: &Vec<i32>, k: i64, c: i64) -> bool {
        candies.iter().map(|candy| *candy as i64 / c).sum::<i64>() >= k
    }

    let mut left = 0;
    let mut right = *candies.iter().max().unwrap() as i64;

    while left < right {
        let mid = (left + right + 1) / 2;
        if allocate_candies(&candies, k, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left as i32
}
