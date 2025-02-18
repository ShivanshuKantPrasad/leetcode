// https://leetcode.com/problems/construct-smallest-number-from-di-string/

pub fn smallest_number(pattern: String) -> String {
    let mut result = String::new();
    let mut num_stack = Vec::<i32>::new();
    let n = pattern.len();

    for index in 0..=n {
        num_stack.push(index as i32 + 1);
        if pattern.chars().nth(index) != Some('D') {
            while let Some(num) = num_stack.pop() {
                result += &num.to_string();
            }
        }
    }

    result
}
