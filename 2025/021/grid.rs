// https://leetcode.com/problems/grid-game/description/

pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    let mut first_row_sum: i64 = grid[0].iter().map(|x| *x as i64).sum();
    let mut second_row_sum = 0;

    let mut minimum_sum = i64::MAX;

    for turn_index in 0..grid[0].len() {
        first_row_sum -= grid[0][turn_index] as i64;
        minimum_sum = minimum_sum.min(first_row_sum.max(second_row_sum));
        second_row_sum += grid[1][turn_index] as i64;
    }
    minimum_sum
}
