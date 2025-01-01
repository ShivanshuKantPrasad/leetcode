// https://leetcode.com/problems/maximum-total-importance-of-roads/

pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut cities = roads
        .into_iter()
        .fold(vec![0; n as usize], |mut acc, road| {
            acc[road[0] as usize] += 1;
            acc[road[1] as usize] += 1;
            acc
        });
    cities.sort_unstable();
    cities
        .iter()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + ((i + 1) * c) as i64)
}
