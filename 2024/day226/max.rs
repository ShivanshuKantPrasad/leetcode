// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/description/

pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    use std::collections::HashSet;

    let banned: HashSet<_> = banned.into_iter().collect();
    let mut sum = 0;
    let mut count = 0;

    for i in 1..=n {
        if banned.contains(&i) {
            continue;
        }
        if sum + i > max_sum {
            break;
        }
        sum += i;
        count += 1;
    }

    count
}
