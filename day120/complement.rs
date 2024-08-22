// https://leetcode.com/problems/number-complement/

pub fn find_complement(num: i32) -> i32 {
    let bits = num.ilog2() + 1;
    let bitmask = (1 << bits) - 1;
    num ^ bitmask
}
