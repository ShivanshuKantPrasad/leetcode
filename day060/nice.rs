// https://leetcode.com/problems/count-number-of-nice-subarrays/

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    at_most(&nums, k) - at_most(&nums, k - 1)
}

fn at_most(nums: &Vec<i32>, k: i32) -> i32 {
    let mut window_size = 0;
    let mut subarrays = 0;
    let mut start = 0;
    for end in 0..nums.len() {
        window_size += nums[end] % 2;
        while window_size > k {
            window_size -= nums[start] % 2;
            start += 1;
        }
        subarrays += end - start + 1;
    }
    subarrays as i32
}

fn main() {
    println!("{}", number_of_subarrays(vec![1, 1, 2, 1, 1], 3));
    println!("{}", number_of_subarrays(vec![2, 4, 6], 1));
    println!(
        "{}",
        number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)
    );
}
