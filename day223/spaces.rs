// https://leetcode.com/problems/adding-spaces-to-a-string/description/

pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut res = String::with_capacity(s.len() + spaces.len());
    let mut prev = 0;
    for i in spaces {
        let i = i as usize;
        res += &s[prev..i];
        res += &" ";
        prev = i;
    }
    res + &s[prev..]
}
