// https://leetcode.com/problems/maximum-score-from-removing-substrings/description/

pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut result = 0;

    let mut stack: Vec<u8> = vec![];
    let mut s = s.as_bytes();

    let hs = (if y > x { b"ba" } else { b"ab" });
    let ls = (if hs == b"ba" { b"ab" } else { b"ba" });

    let max = x.max(y);
    let min = x.min(y);

    for ch in s {
        if stack.len() > 0 && *ch == hs[1] && stack[stack.len() - 1] == hs[0] {
            stack.pop();
            result += max;
        } else {
            stack.push(*ch);
        }
    }

    let mut ns = vec![];
    for ch in stack {
        if ns.len() > 0 && ch == ls[1] && ns[ns.len() - 1] == ls[0] {
            ns.pop();
            result += min;
        } else {
            ns.push(ch);
        }
    }

    result
}
