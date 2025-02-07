// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/description/

pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let num_queries = queries.len();
    let mut balls = HashMap::<i32, i32>::with_capacity(num_queries);
    let mut freqs = HashMap::<i32, i32>::with_capacity(num_queries);

    let mut result = Vec::with_capacity(num_queries);
    for query in &queries {
        let ball_index = query[0];
        let new_color = query[1];

        if let Some(old_color) = balls.insert(ball_index, new_color) {
            *freqs.get_mut(&old_color).unwrap() -= 1;
            if freqs[&old_color] == 0 {
                freqs.remove(&old_color);
            }
        }

        *freqs.entry(new_color).or_default() += 1;
        result.push(freqs.len() as i32);
    }

    result
}
