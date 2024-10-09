// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut stack = 0;
    let mut ans = 0;

    for ch in s.chars() {
        match ch {
            '(' => stack += 1,
            ')' => {
                if stack > 0 {
                    stack -= 1
                } else {
                    ans += 1;
                }
            }
            _ => (),
        }
    }
    ans + stack
}
