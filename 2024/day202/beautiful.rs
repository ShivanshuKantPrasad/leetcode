// https://leetcode.com/problems/most-beautiful-item-for-each-query/description/

pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let n = queries.len();
    let mut ans = vec![0; n];

    items.sort_unstable_by_key(|x| x[0]);

    let mut queries_with_indices = queries.into_iter().enumerate().collect::<Vec<_>>();
    queries_with_indices.sort_unstable_by_key(|q| q.1);

    let mut item_index = 0;
    let mut max_beauty = 0;

    for i in 0..n {
        let query = queries_with_indices[i].1;
        let original_index = queries_with_indices[i].0;

        while item_index < items.len() && items[item_index][0] <= query {
            max_beauty = max_beauty.max(items[item_index][1]);
            item_index += 1;
        }
        ans[original_index] = max_beauty
    }

    ans
}
