use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, usize>::new();
        let mut sum = 0;

        for i in 0..nums.len() {
            sum = (sum + nums[i]) % k;
            if sum == 0 && i > 0 {
                return true;
            }
            if map.contains_key(&sum) {
                if i - map[&sum] > 1 {
                    return true;
                }
            } else {
                map.insert(sum, i);
            }
        }

        return false;
    }
}

fn main() {
    println!("{}", Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    println!("{}", Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    println!("{}", Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    println!("{}", Solution::check_subarray_sum(vec![23, 2, 4, 6, 6], 7));
    println!("{}", Solution::check_subarray_sum(vec![2, 4, 3], 6));
}
