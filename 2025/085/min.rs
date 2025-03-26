// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/description/

pub fn min_operations(mut grid: Vec<Vec<i32>>, x: i32) -> i32 {
    use std::collections::HashSet;

    let mut grid = grid.into_iter().flatten().collect::<Vec<_>>();

    grid.sort_unstable();
    if grid
        .iter()
        .map(|num| *num % x)
        .collect::<HashSet<_>>()
        .len()
        != 1
    {
        return -1;
    }

    let median = grid[grid.len() / 2];
    grid.iter().map(|num| ((*num - median) / x).abs()).sum()
}
