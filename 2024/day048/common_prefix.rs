// https://leetcode.com/problems/longest-common-prefix/description/

struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, cur| {
                acc.chars()
                    .zip(cur.chars())
                    .take_while(|(a, c)| a == c)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ])
    );
}
