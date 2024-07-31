// https://leetcode.com/problems/filling-bookcase-shelves/description/

pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut dp = vec![0; books.len() + 1];

    dp[0] = 0;
    dp[1] = books[0][1];

    for i in 1..=books.len() {
        let mut remaining_shelf_width = shelf_width - books[i - 1][0];
        let mut max_height = books[i - 1][1];
        dp[i] = dp[i - 1] + max_height;

        let mut j = i - 1;
        while j > 0 && remaining_shelf_width - books[j - 1][0] >= 0 {
            remaining_shelf_width -= books[j - 1][0];
            max_height = max_height.max(books[j - 1][1]);
            dp[i] = dp[i].min(dp[j - 1] + max_height);
            j -= 1;
        }
    }

    return dp[books.len()];
}
