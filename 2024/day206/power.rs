// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/

pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums.windows(k as usize)
        .map(|w| {
            if w.windows(2).all(|e| e[1] - e[0] == 1) {
                *w.last().unwrap()
            } else {
                -1
            }
        })
        .collect()
}
