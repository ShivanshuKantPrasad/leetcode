// https://leetcode.com/problems/separate-black-and-white-balls/description/

pub fn minimum_steps(s: String) -> i64 {
    s.chars()
        .fold((0, 0), |(total_swaps, black_ball), ch| {
            if ch == '0' {
                (total_swaps + black_ball, black_ball)
            } else {
                (total_swaps, black_ball + 1)
            }
        })
        .0
}
