// https://leetcode.com/problems/apply-operations-to-an-array/description/

pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut write_index = 0;

    for index in 0..n {
        if index < n - 1 && nums[index] == nums[index + 1] {
            nums[index] *= 2;
            nums[index + 1] = 0;
        }

        if nums[index] != 0 {
            if index != write_index {
                nums.swap(index, write_index);
            }
            write_index += 1;
        }
    }

    nums
}
