// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/description/

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 2..nums.len() {
        if (nums[i - 2] == 0) {
            count += 1;

            nums[i - 2] ^= 1;
            nums[i - 1] ^= 1;
            nums[i] ^= 1;
        }
    }

    let sum = nums.iter().sum::<i32>() as usize;
    if sum == nums.len() {
        count
    } else {
        -1
    }
}
