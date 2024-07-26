// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/submissions/1333594156/

use std::cmp::Reverse;

pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let mut distance = vec![vec![i32::MAX; n as usize]; n as usize];

    for edge in edges {
        distance[edge[0] as usize][edge[1] as usize] = edge[2];
        distance[edge[1] as usize][edge[0] as usize] = edge[2];
    }
    for i in 0..n {
        distance[i as usize][i as usize] = 0;
    }

    for via in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distance[i as usize][via as usize] == i32::MAX
                    || distance[via as usize][j as usize] == i32::MAX
                {
                    continue;
                }
                distance[i as usize][j as usize] = std::cmp::min(
                    distance[i as usize][via as usize] + distance[via as usize][j as usize],
                    distance[i as usize][j as usize],
                );
            }
        }
    }

    distance
        .iter()
        .map(|x| x.iter().filter(|&&x| x <= distance_threshold).count() as i32 - 1)
        .enumerate()
        .min_by_key(|&(node, number)| (number, Reverse(node as i32)))
        .map(|(node, _)| node as i32)
        .unwrap()
}
