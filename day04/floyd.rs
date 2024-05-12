// https://leetcode.com/problems/sum-of-distances-in-tree/description/
//
struct Solution;
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut distance = vec![vec![100000; n as usize]; n as usize];
        for i in 0..n as usize {
            for j in 0..n as usize {
                if i == j {
                    distance[i][j] = 0;
                }
                if edges.contains(&vec![i as i32, j as i32]) {
                    distance[i][j] = 1;
                    distance[j][i] = 1;
                }
            }
        }
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                    distance[j][i] = distance[i][j];
                }
            }
        }
        distance.iter().map(|x| x.iter().sum()).collect::<Vec<_>>()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sum_of_distances_in_tree(
            6,
            [
                [0, 1].to_vec(),
                [0, 2].to_vec(),
                [2, 3].to_vec(),
                [2, 4].to_vec(),
                [2, 5].to_vec(),
            ]
            .to_vec(),
        )
    );
}
