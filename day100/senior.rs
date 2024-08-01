// https://leetcode.com/problems/number-of-senior-citizens/

pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .map(|x| x[11..13].parse::<i32>().unwrap())
        .filter(|x| *x > 60)
        .count() as i32
}
