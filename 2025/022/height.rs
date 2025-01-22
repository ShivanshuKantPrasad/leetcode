// https://leetcode.com/problems/map-of-highest-peak/description/

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut height = vec![vec![i32::MAX; n]; m];

    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                height[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;
        let mut min_height = i32::MAX;

        if row != 0 {
            min_height = min_height.min(height[row - 1][col]);
            queue.push_back((row - 1, col));
        }

        if col != n - 1 {
            min_height = min_height.min(height[row][col + 1]);
            queue.push_back((row, col + 1));
        }

        if col != 0 {
            min_height = min_height.min(height[row][col - 1]);
            queue.push_back((row, col - 1));
        }

        if row != m - 1 {
            min_height = min_height.min(height[row + 1][col]);
            queue.push_back((row + 1, col));
        }

        if min_height != i32::MAX && height[row][col] != 0 {
            height[row][col] = min_height + 1;
        }
    }

    height
}
