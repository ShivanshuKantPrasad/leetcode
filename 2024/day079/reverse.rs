// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/description/

pub fn reverse_parentheses(s: String) -> String {
    let mut stack = Vec::new();
    let mut result = Vec::new();

    for c in s.chars() {
        match c {
            '(' => {
                stack.push(result.len());
            }
            ')' => {
                let start = stack.pop().unwrap();
                let end = result.len();
                result[start..end].reverse();
            }
            x => result.push(x),
        }
    }

    result.iter().collect()
}

fn main() {
    println!("{}", reverse_parentheses("(abcd)".to_string()));
    println!("{}", reverse_parentheses("(u(love)i)".to_string()));
    println!("{}", reverse_parentheses("(ed(et(oc))el)".to_string()));
}
