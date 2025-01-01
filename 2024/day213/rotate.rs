// https://leetcode.com/problems/rotating-the-box/description/

pub fn rotate_the_box(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = input.len();
    let n = input[0].len();
    let mut rotated = vec![vec!['.'; m]; n];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let ch = input[i][j];
            let mut new_j = j;
            if ch == '#' {
                while new_j + 1 < n && rotated[new_j + 1][m - i - 1] == '.' {
                    new_j += 1;
                }
            }

            rotated[new_j][m - i - 1] = ch;
        }
    }

    rotated
}
