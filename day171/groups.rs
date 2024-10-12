// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/submissions/1419579533/

pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut events = vec![];

    for interval in intervals {
        events.push((interval[0], 1));
        events.push((interval[1] + 1, -1));
    }

    events.sort_unstable();

    let mut concurrent_intervals = 0;
    let mut max = 0;
    for event in events {
        concurrent_intervals += event.1;
        max = max.max(concurrent_intervals);
    }
    max
}
