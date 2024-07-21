// https://leetcode.com/problems/build-a-matrix-with-conditions/description/

pub fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    fn topo_sort(edges: Vec<Vec<i32>>, mut n: usize) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut adj = vec![vec![0; n + 1]; n + 1];

        let mut deg = vec![0; n + 1];
        let mut order = vec![0; n];

        let mut idx = 0;
        for x in edges {
            adj[x[0] as usize].push(x[1]);
            deg[x[1] as usize] += 1;
        }

        let mut q = VecDeque::new();
        for i in 1..=n {
            if deg[i] == 0 {
                q.push_back(i as i32);
            }
        }

        while !q.is_empty() {
            let f = q.pop_front().unwrap();
            order[idx] = f;
            idx += 1;
            n -= 1;
            for v in adj[f as usize].clone() {
                deg[v as usize] -= 1;
                if deg[v as usize] == 0 {
                    q.push_back(v);
                }
            }
        }

        if n != 0 {
            return vec![];
        }
        order
    }

    let k = k as usize;
    let order_rows = topo_sort(row_conditions, k);
    let order_columns = topo_sort(col_conditions, k);

    if order_rows.len() == 0 || order_columns.len() == 0 {
        return vec![];
    }

    let mut matrix = vec![vec![0; k]; k];

    for i in 0..k {
        for j in 0..k {
            if order_rows[i] == order_columns[j] {
                matrix[i][j] = order_rows[i];
            }
        }
    }
    matrix
}
