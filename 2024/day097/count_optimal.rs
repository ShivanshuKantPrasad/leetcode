pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut count = 0;

    // Select the middle soldiers.
    for j in 1..rating.len() - 1 {
        let mut greater_i = 0;
        let mut lesser_i = 0;

        // Count the number of possibe rating[i]'s that are smaller and greater than rating[j]
        for i in 0..j {
            if rating[i] < rating[j] {
                lesser_i += 1;
            } else {
                greater_i += 1;
            }
        }

        let mut greater_k = 0;
        let mut lesser_k = 0;

        // Count the number of possibe rating[k]'s that are smaller and greater than rating[j]
        for k in j + 1..rating.len() {
            if rating[k] > rating[j] {
                greater_k += 1;
            } else {
                lesser_k += 1;
            }
        }

        // Select one i such rating[i] < rating[j] and one k such that rating[j] < rating[k]
        // This can be done in lesser_i * greater_k ways
        // similarly rating[i] > rating[j] > rating[k] can be done in greater_i * lesser_k ways
        // add them to total count
        count += lesser_i * greater_k + greater_i * lesser_k;
    }
    count
}
