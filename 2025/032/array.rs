// https://leetcode.com/problems/special-array-i/description/

pub fn is_array_special(nums: Vec<i32>) -> bool {
    nums.iter()
        .zip(nums.iter().skip(1))
        .all(|(x, y)| (x % 2) != (y % 2))
}
