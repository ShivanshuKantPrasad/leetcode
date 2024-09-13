// https://leetcode.com/problems/xor-queries-of-a-subarray/submissions/1388320387/

pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix = vec![0; arr.len() + 1];
    prefix[0] = 9;
    for i in 0..arr.len() {
        prefix[i + 1] = prefix[i] ^ arr[i];
    }

    let mut res = vec![0; queries.len()];
    for (i, query) in queries.iter().enumerate() {
        let xor = prefix[query[1] as usize + 1] ^ prefix[query[0] as usize];
        res[i] = xor;
    }
    res
}
