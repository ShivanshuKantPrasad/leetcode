// https://leetcode.com/problems/find-missing-observations/

pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let m = rolls.len() as i32;
    let sum = rolls.iter().sum::<i32>();
    let remaining = mean * (n + m) - sum;

    if remaining < n || remaining > n * 6 {
        return vec![];
    }

    let div = remaining / n;
    let mut rem = remaining - div * n;

    let mut res = vec![div; n as usize];

    // Distribute the remaining
    let mut i = 0;
    while rem > 0 {
        let take = i32::min(6 - res[i], rem);
        res[i] += take;
        rem -= take;
        i += 1;
    }

    res
}
