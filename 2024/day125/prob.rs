// https://leetcode.com/problems/path-with-maximum-probability/

pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start_node: i32,
    end_node: i32,
) -> f64 {
    let mut max_prob = vec![0f64; n as usize];
    max_prob[start_node as usize] = 1.0;

    for i in 0..n - 1 {
        let mut has_update = false;
        for j in 0..edges.len() {
            let u = edges[j][0] as usize;
            let v = edges[j][1] as usize;
            let path_prob = succ_prob[j];
            if max_prob[u] * path_prob > max_prob[v] {
                max_prob[v] = max_prob[u] * path_prob;
                has_update = true;
            }
            if max_prob[v] * path_prob > max_prob[u] {
                max_prob[u] = max_prob[v] * path_prob;
                has_update = true;
            }
        }
        if !has_update {
            break;
        }
    }
    max_prob[end_node as usize]
}
