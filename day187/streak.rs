// https://leetcode.com/problems/longest-square-streak-in-an-array/

pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let mut processed = std::collections::HashSet::new();

    let mut res = 0;

    for &current in &nums {
        if processed.contains(&current) {
            continue;
        }

        let mut streak = current;
        let mut streak_length = 1;

        loop {
            if (streak as i64 * streak as i64 > 100000) {
                break;
            };

            if nums.binary_search(&(streak * streak)).is_ok() {
                streak *= streak;
                processed.insert(streak);
                streak_length += 1;
            } else {
                break;
            }
        }
        res = res.max(streak_length);
    }

    if res < 2 {
        -1
    } else {
        res
    }
}
