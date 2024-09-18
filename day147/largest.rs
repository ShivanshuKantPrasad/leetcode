// https://leetcode.com/problems/largest-number/

pub fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_unstable_by(|a, b| format!("{b}{a}").cmp(&format!("{a}{b}")));
    if nums[0] == 0 {
        nums[0].to_string();
    } else {
        nums.into_iter().map(|x| x.to_string()).collect()
    }
}
