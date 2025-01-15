// https://leetcode.com/problems/minimize-xor/description/

pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut set_bits = num2.count_ones();

    let mut res = 0;

    for i in (0..31).rev() {
        if (num1 & (1 << i) != 0) && set_bits != 0 {
            res |= 1 << i;
            set_bits -= 1;
        }
    }

    for i in 0..31 {
        if set_bits != 0 && (res & (1 << i) == 0) {
            res |= 1 << i;
            set_bits -= 1;
        }
    }

    res
}
