// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description/

pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let mut flipped = vec![false; nums.len()];
    let mut valid_flips = 0;
    let mut flip_count = 0;

    for i in 0..nums.len() {
        if i >= k as usize {
            if flipped[i - k as usize] {
                valid_flips -= 1;
            }
        }

        if valid_flips % 2 == nums[i] {
            if i + k as usize > nums.len() {
                return -1;
            }
            valid_flips += 1;
            flipped[i] = true;
            flip_count += 1;
        }
    }
    return flip_count;
}

fn main() {
    println!("{}", min_k_bit_flips(vec![0, 1, 0], 1));
    println!("{}", min_k_bit_flips(vec![1, 1, 0], 2));
    println!("{}", min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3));
}
