// https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/description/

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = nums
        .into_iter()
        .map(|x| Reverse(x as i64))
        .collect::<BinaryHeap<_>>();
    let mut n = 0;
    loop {
        let Reverse(smallest) = heap.pop().unwrap();
        if smallest >= k as i64 {
            return n;
        }
        let Reverse(second) = heap.pop().unwrap();
        heap.push(Reverse(smallest * 2 + second));
        n += 1;
    }
}
