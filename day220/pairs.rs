// https://leetcode.com/problems/valid-arrangement-of-pairs/description/

pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, VecDeque};

    let mut adjacency_matrix = HashMap::<i32, VecDeque<i32>>::new();
    let mut in_degree = HashMap::<i32, i32>::new();
    let mut out_degree = HashMap::<i32, i32>::new();

    for pair in &pairs {
        let start = pair[0];
        let end = pair[1];
        adjacency_matrix.entry(start).or_default().push_back(end);
        *out_degree.entry(start).or_insert(0) += 1;
        *in_degree.entry(end).or_insert(0) += 1;
    }

    let mut result = Vec::new();

    let mut start_node = -1;
    for (&node, &_) in &out_degree {
        if *out_degree.get(&node).unwrap_or(&0) == in_degree.get(&node).unwrap_or(&0) + 1 {
            start_node = node;
            break;
        }
    }

    if start_node == -1 {
        start_node = pairs[0][0];
    }

    let mut node_stack = Vec::new();

    node_stack.push(start_node);

    while let Some(&node) = node_stack.last() {
        match adjacency_matrix.get_mut(&node).and_then(|x| x.pop_front()) {
            Some(next_node) => node_stack.push(next_node),
            None => {
                result.push(node);
                node_stack.pop();
            }
        }
    }

    result.reverse();

    let mut paired_result = Vec::new();

    for i in 1..result.len() {
        paired_result.push(vec![result[i - 1], result[i]]);
    }

    paired_result
}
