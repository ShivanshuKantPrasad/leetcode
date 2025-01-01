// https://leetcode.com/problems/two-best-non-overlapping-events/description/

pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    let mut times = Vec::new();

    for e in events {
        times.push(vec![e[0], 1, e[2]]);
        times.push(vec![e[1] + 1, 0, e[2]]);
    }

    let mut ans = 0;
    let mut max_value = 0;
    times.sort_unstable();

    for time in times {
        if time[1] == 1 {
            ans = ans.max(time[2] + max_value);
        } else {
            max_value = max_value.max(time[2]);
        }
    }
    ans
}
