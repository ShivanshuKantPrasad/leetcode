// https://leetcode.com/problems/find-eventual-safe-states/description/

pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let terminal = graph
        .iter()
        .enumerate()
        .filter(|(i, edges)| edges.len() == 0)
        .map(|(i, _)| i as i32)
        .collect::<Vec<_>>();

    println!("{terminal:?}");

    let n = graph.len();
    let mut in_degree = vec![0; n];
    let mut adj = vec![vec![]; n];

    for i in 0..n {
        for &node in &graph[i] {
            adj[node as usize].push(i);
            in_degree[i] += 1;
        }
    }

    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();

    for i in 0..n {
        if in_degree[i] == 0 {
            q.push(i);
        }
    }

    let mut safe = vec![false; n];
    while let Some(node) = q.pop() {
        safe[node] = true;

        for &neighbor in &adj[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                q.push(neighbor);
            }
        }
    }

    safe.iter()
        .enumerate()
        .filter_map(|(i, is_safe)| if *is_safe { Some(i as i32) } else { None })
        .collect()
}
