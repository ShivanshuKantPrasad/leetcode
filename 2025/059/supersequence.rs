// https://leetcode.com/problems/shortest-common-supersequence/description/

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let n1 = str1.len();
    let n2 = str2.len();

    let str1 = str1.chars().collect::<Vec<_>>();
    let str2 = str2.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

    for row in 0..=n1 {
        dp[row][0] = row;
    }

    for col in 0..=n2 {
        dp[0][col] = col;
    }

    for row in 1..=n1 {
        for col in 1..=n2 {
            if str1[row - 1] == str2[col - 1] {
                dp[row][col] = dp[row - 1][col - 1] + 1;
            } else {
                dp[row][col] = dp[row - 1][col].min(dp[row][col - 1]) + 1;
            }
        }
    }

    let mut supersequence = String::new();
    let mut row = n1;
    let mut col = n2;

    while row > 0 && col > 0 {
        if str1[row - 1] == str2[col - 1] {
            supersequence.push(str1[row - 1]);
            row -= 1;
            col -= 1;
        } else if dp[row - 1][col] < dp[row][col - 1] {
            supersequence.push(str1[row - 1]);
            row -= 1;
        } else {
            supersequence.push(str2[col - 1]);
            col -= 1;
        }
    }

    while row > 0 {
        supersequence.push(str1[row - 1]);
        row -= 1;
    }

    while col > 0 {
        supersequence.push(str2[col - 1]);
        col -= 1;
    }

    supersequence.chars().rev().collect()
}
