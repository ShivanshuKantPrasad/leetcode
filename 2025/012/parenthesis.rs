// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/description/

pub fn can_be_valid(s: String, locked: String) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    if n % 2 == 1 {
        return false;
    }
    let locked = locked.as_bytes();

    let mut unlocked = Vec::new();
    let mut open_brackets = Vec::new();

    for i in 0..n {
        if locked[i] == b'0' {
            unlocked.push(i);
        } else if s[i] == b'(' {
            open_brackets.push(i);
        } else if s[i] == b')' {
            if !open_brackets.is_empty() {
                open_brackets.pop();
            } else if !unlocked.is_empty() {
                unlocked.pop();
            } else {
                return false;
            }
        }
    }

    while let (Some(&open_top), Some(&unlock_top)) = (open_brackets.last(), unlocked.last()) {
        if open_top < unlock_top {
            open_brackets.pop();
            unlocked.pop();
        } else {
            break;
        }
    }

    open_brackets.is_empty()
}
