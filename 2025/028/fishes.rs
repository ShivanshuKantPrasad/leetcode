// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/

pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_fish = 0;

    let mut visited = vec![vec![false; cols]; rows];

    fn calculate_fishes(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        row: usize,
        col: usize,
    ) -> i32 {
        if row < 0
            || row >= grid.len()
            || col < 0
            || col >= grid[0].len()
            || grid[row][col] == 0
            || visited[row][col]
        {
            return 0;
        }

        visited[row][col] = true;

        grid[row][col]
            + calculate_fishes(grid, visited, row + 1, col)
            + calculate_fishes(grid, visited, row - 1, col)
            + calculate_fishes(grid, visited, row, col + 1)
            + calculate_fishes(grid, visited, row, col - 1)
    }

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] > 0 && !visited[row][col] {
                max_fish = max_fish.max(calculate_fishes(&grid, &mut visited, row, col));
            }
        }
    }

    max_fish
}
