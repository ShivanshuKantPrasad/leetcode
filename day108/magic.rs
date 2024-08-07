// https://leetcode.com/problems/magic-squares-in-grid/

fn is_magic_square(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let sequence = "2943816729438167";
    let sequence_reversed = "7618349276183492";

    let mut border = String::new();
    let border_indices = vec![0, 1, 2, 5, 8, 7, 6, 3];

    for i in border_indices {
        let num = grid[row + i / 3][col + (i % 3)];
        border += &num.to_string();
    }

    grid[row][col] % 2 == 0
        && grid[row + 1][col + 1] == 5
        && (sequence.find(&border).is_some() || sequence_reversed.find(&border).is_some())
}
pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    if m < 3 || n < 3 {
        return 0;
    }
    for row in 0..(m - 2) {
        for col in 0..(n - 2) {
            println!("{row} {col}");
            if is_magic_square(&grid, row, col) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]])
    );
    println!("{}", num_magic_squares_inside(vec![vec![8]]));
}
