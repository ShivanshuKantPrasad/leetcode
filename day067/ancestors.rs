// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/description/

use std::collections::HashSet;
pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut adjacency_list = vec![Vec::new(); n as usize];

    for edge in edges {
        adjacency_list[edge[1] as usize].push(edge[0]);
    }

    let mut ancestors_list = Vec::new();

    for i in 0..n {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        find_children(i, &adjacency_list, &mut visited);

        for node in 0..n {
            if node == i {
                continue;
            }
            if visited.contains(&node) {
                ancestors.push(node);
            }
        }
        ancestors_list.push(ancestors);
    }

    return ancestors_list;
}

pub fn find_children(
    current_node: i32,
    adjacency_list: &Vec<Vec<i32>>,
    visited_nodes: &mut HashSet<i32>,
) {
    visited_nodes.insert(current_node);
    for neighbour in &adjacency_list[current_node as usize] {
        if !visited_nodes.contains(neighbour) {
            find_children(*neighbour, adjacency_list, visited_nodes);
        }
    }
}

fn main() {
    println!(
        "{:?}",
        get_ancestors(
            8,
            vec![
                vec![0, 3],
                vec![0, 4],
                vec![1, 3],
                vec![2, 4],
                vec![2, 7],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![4, 6]
            ]
        )
    );
    println!(
        "{:?}",
        get_ancestors(
            5,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![0, 4],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        )
    );
}
