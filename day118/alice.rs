//  https://leetcode.com/problems/stone-game-ii/submissions/1362390845/

pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    let mut suffix_sum = vec![0; n + 1];

    for i in (0..n).rev() {
        suffix_sum[i] = suffix_sum[i + 1] + piles[i];
    }

    for i in (0..n + 1) {
        dp[i][n] = suffix_sum[i]
    }

    for i in (0..n).rev() {
        for max_till_now in (1..n).rev() {
            for x in (1..usize::min(2 * max_till_now, n - i) + 1) {
                dp[i][max_till_now] =
                    dp[i][max_till_now].max(suffix_sum[i] - dp[i + x][x.max(max_till_now)]);
            }
        }
    }
    dp[0][1]
}
