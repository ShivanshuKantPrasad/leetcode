// https://leetcode.com/problems/neighboring-bitwise-xor/description/

pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    derived.iter().fold(0, |acc, num| acc ^ num) == 0
}
