// https://leetcode.com/problems/minimum-time-difference/submissions/1392392316/

pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut res = time_points
        .iter()
        .map(|x| x.split_once(":").unwrap())
        .map(|(h, m)| (h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    res.sort_unstable();

    let mut min = res[res.len() - 1] - res[0];
    let mut max = res[res.len() - 1] - res[0];

    for arr in res.windows(2) {
        min = min.min(arr[1] - arr[0]);
        max = max.max(arr[1] - arr[0]);
    }

    i32::min(min, 1440 - max)
}
