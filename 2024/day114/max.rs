// https://leetcode.com/problems/maximum-distance-in-arrays/description/

pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut lo = 10000;
    let mut hi = -10000;
    arrays
        .iter()
        .map(|x| (x[0], x[x.len() - 1]))
        .fold(0, |ans, (x, y)| {
            let ans = ans.max(hi - x).max(y - lo);
            lo = lo.min(x);
            hi = hi.max(y);
            ans
        })
}
