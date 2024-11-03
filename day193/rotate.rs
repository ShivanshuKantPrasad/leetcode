// https://leetcode.com/problems/rotate-string/description/

pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len()
        && (0..s.len()).any(|i| {
            s.chars()
                .cycle()
                .skip(i)
                .zip(goal.chars())
                .all(|(a, b)| a == b)
        })
}
