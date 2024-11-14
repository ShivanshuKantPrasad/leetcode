// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/?envType=daily-question&envId=2024-11-14

pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    fn can_distribute(n: i32, quantities: &[i32], max: u32) -> bool {
        quantities
            .iter()
            .map(|quantity| (*quantity as u32).div_ceil(max))
            .sum::<u32>()
            <= n as u32
    }

    let mut left = 1;
    let mut right = *quantities.iter().max().unwrap();

    while left < right {
        let mid = (left + right) / 2;
        if can_distribute(n, &quantities, mid as u32) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}
