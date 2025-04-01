// https://leetcode.com/problems/solving-questions-with-brainpower/

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {

    let n = questions.len();
    let mut dp = vec![0; n];

    for i in (0..n).rev() {
        let points = questions[i][0] as i64;
        let brainpower = questions[i][1] as usize;

        let next = brainpower + i + 1;
        
        // Solve
        if next < n {
            dp[i] = points + dp[next];
        } else {
            dp[i] = points;
        }

        // Skip
        if i + 1 < n {
            dp[i] = dp[i].max(dp[i + 1]);
        }
    }

    dp[0]
}
