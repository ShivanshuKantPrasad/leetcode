// https://leetcode.com/problems/count-days-without-meetings/description/

pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();

    let mut merged = Vec::new();
    merged.push(meetings[0].clone());

    for i in 1..meetings.len() {
        let last: &mut Vec<i32> = merged.last_mut().unwrap();
        if last[0] <= meetings[i][0] && last[1] >= meetings[i][0] {
            last[1] = meetings[i][1].max(last[1]);
        } else {
            merged.push(meetings[i].clone());
        }
    }

    let meeting_days = merged.into_iter().map(|x| x[1] - x[0] + 1).sum::<i32>();
    days - meeting_days
}
