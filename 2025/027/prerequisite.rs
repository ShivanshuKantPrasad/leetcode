// https://leetcode.com/problems/course-schedule-iv/submissions/1521736928/

pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let n = num_courses as usize;
    let mut is_prerequisite = vec![vec![false; n]; n];

    for req in &prerequisites {
        is_prerequisite[req[0] as usize][req[1] as usize] = true;
    }

    for intermediate in 0..n {
        for src in 0..n {
            for target in 0..n {
                is_prerequisite[src][target] = is_prerequisite[src][target]
                    || (is_prerequisite[src][intermediate]
                        && is_prerequisite[intermediate][target]);
            }
        }
    }

    queries
        .iter()
        .map(|q| is_prerequisite[q[0] as usize][q[1] as usize])
        .collect()
}
