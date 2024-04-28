struct Solution;
impl Solution {
    fn dfs(
        i: usize,
        p: usize,
        neighbors: &Vec<Vec<usize>>,
        res: &mut Vec<i32>,
        count: &mut Vec<i32>,
    ) {
        for &u in &neighbors[i] {
            if u == p {
                continue;
            }
            Self::dfs(u, i, neighbors, res, count);
            count[i] += count[u];
            res[i] += res[u] + count[u];
        }
        count[i] += 1;
    }

    fn dfs2(
        i: usize,
        n: i32,
        p: usize,
        neighbors: &Vec<Vec<usize>>,
        res: &mut Vec<i32>,
        count: &mut Vec<i32>,
    ) {
        for &u in &neighbors[i] {
            if u == p {
                continue;
            }
            res[u] = res[i] - count[u] + n - count[u];
            Self::dfs2(u, n, i, neighbors, res, count);
        }
    }
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let mut neighbors = vec![Vec::<usize>::new(); n as usize];
        let mut count = vec![0; n as usize];
        for x in edges {
            let i = x[0] as usize;
            let j = x[1] as usize;
            neighbors[i].push(j);
            neighbors[j].push(i);
        }

        Self::dfs(0, 100000, &neighbors, &mut res, &mut count);
        Self::dfs2(0, n, 100000, &neighbors, &mut res, &mut count);
        res
    }
}

fn main() {
    // println!(
    //     "{:?}",
    for _ in 0..100000 {
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
        );
    }
    // );
}
