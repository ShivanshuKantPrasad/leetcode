// https://leetcode.com/problems/maximum-xor-for-each-query/description/

pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    nums.into_iter()
        .scan(0, |xor, num| {
            *xor ^= num;
            Some(((1 << maximum_bit) - 1) ^ *xor)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}
