struct Solution;
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut operations = 0;
        let mut carry = 0;
        for i in (1..s.len()).rev() {
            if (s.as_bytes()[i] - b'0' + carry) % 2 != 0 {
                operations += 2;
                carry = 1;
            } else {
                operations += 1;
            }
        }
        return operations + carry as i32;
    }
}

fn main() {
    println!("{}", Solution::num_steps("1101".to_string()));
    println!("{}", Solution::num_steps("10".to_string()));
    println!("{}", Solution::num_steps("1".to_string()));
}
