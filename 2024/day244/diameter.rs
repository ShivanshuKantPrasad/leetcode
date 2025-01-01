// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/description/
use std::collections::VecDeque;

struct Solution;
impl Solution {
    fn build_adj_list(edges: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
        let mut result = vec![Vec::new(); n];
        for edge in edges {
            let a = edge[0];
            let b = edge[1];
            result[a as usize].push(b);
            result[b as usize].push(a);
        }
        result
    }

    fn bfs(adj_list: &Vec<Vec<i32>>, start: i32) -> (i32, i32) {
        let mut dist = vec![None; adj_list.len()];
        let mut queue = VecDeque::new();
        dist[start as usize] = Some(0);
        queue.push_back(start);

        let mut farthest_node = start;
        let mut max_dist = 0;

        while let Some(node) = queue.pop_front() {
            let current_dist = dist[node as usize].unwrap();
            for &neighbor in &adj_list[node as usize] {
                if dist[neighbor as usize].is_none() {
                    dist[neighbor as usize] = Some(current_dist + 1);
                    queue.push_back(neighbor);

                    if current_dist + 1 > max_dist {
                        max_dist = current_dist + 1;
                        farthest_node = neighbor;
                    }
                }
            }
        }
        (farthest_node, max_dist)
    }

    fn tree_diameter(edges: &Vec<Vec<i32>>, n: usize) -> i32 {
        if n == 0 {
            return 0;
        }
        let adj_list = Self::build_adj_list(edges, n);
        let (farthest_node, _) = Self::bfs(&adj_list, 0);
        let (_, diameter) = Self::bfs(&adj_list, farthest_node);
        diameter
    }
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Self::tree_diameter(&edges1, edges1.len() + 1);
        let d2 = Self::tree_diameter(&edges2, edges2.len() + 1);
        (d1.max(d2)).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
    }
}
