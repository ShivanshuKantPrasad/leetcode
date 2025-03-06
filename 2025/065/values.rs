// https://leetcode.com/problems/find-missing-and-repeated-values/editorial/

pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut nums = vec![false; n * n + 1];

    let mut double = 0;
    let mut never = 0;

    for i in 0..n {
        for j in 0..n {
            if nums[grid[i][j] as usize] == true {
                double = grid[i][j];
            }
            nums[grid[i][j] as usize] = true;
        }
    }

    for i in 1..=(n * n) {
        if nums[i] == false {
            never = i as i32;
            break;
        }
    }

    vec![double, never]
}
