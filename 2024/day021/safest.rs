// https://leetcode.com/problems/find-the-safest-path-in-a-grid/

use std::collections::{BinaryHeap, VecDeque};

struct Solution;
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut multi_source_queue = VecDeque::default();

        let mut grid = grid;

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    multi_source_queue.push_back([i, j]);
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            }
        }

        let dir: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

        while !multi_source_queue.is_empty() {
            let mut size = multi_source_queue.len();
            while size > 0 {
                let curr = multi_source_queue.pop_front().unwrap();

                for d in dir {
                    let di = d[0] + curr[0] as i32;
                    let dj = d[1] + curr[1] as i32;

                    let val = grid[curr[0]][curr[1]];

                    if n as i32 > di
                        && di >= 0
                        && n as i32 > dj
                        && dj >= 0
                        && grid[di as usize][dj as usize] == -1
                    {
                        grid[di as usize][dj as usize] = val + 1;
                        multi_source_queue.push_back([di as usize, dj as usize]);
                    }
                }

                size -= 1;
            }
        }

        let mut pq = BinaryHeap::new();
        pq.push([grid[0][0], 0, 0]);
        grid[0][0] = -1;

        while !pq.is_empty() {
            let curr = pq.pop().unwrap();

            if curr[1] == n as i32 - 1 && curr[2] == n as i32 - 1 {
                return curr[0];
            }

            for d in dir {
                let di = d[0] + curr[1];
                let dj = d[1] + curr[2];

                if n as i32 > di
                    && di >= 0
                    && n as i32 > dj
                    && dj >= 0
                    && grid[di as usize][dj as usize] != -1
                {
                    pq.push([curr[0].min(grid[di as usize][dj as usize]), di, dj]);
                    grid[di as usize][dj as usize] = -1;
                }
            }
        }

        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]])
    );
    println!(
        "{}",
        Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]])
    );
    println!(
        "{}",
        Solution::maximum_safeness_factor(vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0]
        ])
    );
}
