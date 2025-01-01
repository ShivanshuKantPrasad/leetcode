// https://leetcode.com/problems/find-champion-ii/description/

pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; n as usize];

    for edge in edges {
        in_degree[edge[1] as usize] += 1;
    }

    if in_degree.iter().filter(|x| **x == 0).count() == 1 {
        in_degree.iter().position(|x| *x == 0).unwrap() as i32
    } else {
        -1
    }
}
