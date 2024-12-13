// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/description/

pub fn find_score(nums: Vec<i32>) -> i64 {
    let mut stack = vec![];
    let mut res = 0;

    for i in 0..nums.len() {
        if stack.is_empty() || nums[i] < *stack.last().unwrap() {
            stack.push(nums[i])
        } else {
            while let Some(x) = stack.pop() {
                res += x as i64;
                stack.pop();
            }
        }
    }

    while let Some(x) = stack.pop() {
        res += x as i64;
        stack.pop();
    }
    res
}
