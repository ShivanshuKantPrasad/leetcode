struct Solution;
impl Solution {
    fn dfs_backtrack(grid: &mut Vec<Vec<i32>>, rows: i32, cols: i32, row: i32, col: i32) -> i32 {
        let DIRECTIONS = [0, 1, 0, -1, 0];
        if (row < 0
            || col < 0
            || row == rows
            || col == cols
            || grid[row as usize][col as usize] == 0)
        {
            return 0;
        }

        let mut max_gold = 0;
        let original_val = grid[row as usize][col as usize];
        grid[row as usize][col as usize] = 0;

        for i in 0..4 {
            max_gold = max_gold.max(Self::dfs_backtrack(
                grid,
                rows,
                cols,
                row + DIRECTIONS[i],
                col + DIRECTIONS[i + 1],
            ));
        }
        grid[row as usize][col as usize] = original_val;
        max_gold + original_val
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut grid = grid;

        let mut max_gold = 0;

        for row in 0..rows {
            for col in 0..cols {
                max_gold = max_gold.max(Self::dfs_backtrack(&mut grid, rows, cols, row, col));
            }
        }
        max_gold
    }
}

pub fn main() {
    println!(
        "{}",
        Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]])
    );
    println!(
        "{}",
        Solution::get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20]
        ])
    );
}
