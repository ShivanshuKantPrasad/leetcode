// https://leetcode.com/problems/minimum-time-to-visit-a-cell-in-a-grid/description/

pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = vec![vec![false; cols]; rows];

    fn is_valid(visited: &Vec<Vec<bool>>, row: isize, col: isize) -> bool {
        row >= 0
            && col >= 0
            && (row as usize) < visited.len()
            && (col as usize) < visited[0].len()
            && !visited[row as usize][col as usize]
    }

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((grid[0][0], 0, 0)));

    while let Some(Reverse((time, row, col))) = pq.pop() {
        if row == rows - 1 && col == cols - 1 {
            return time;
        }

        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        for &(dr, dc) in &directions {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            if !is_valid(&visited, nr, nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let wait_time = if (grid[nr][nc] - time) % 2 == 0 { 1 } else { 0 };
            let next_time = i32::max(grid[nr][nc] + wait_time, time + 1);
            pq.push(Reverse((next_time, nr, nc)));
        }
    }
    -1
}
