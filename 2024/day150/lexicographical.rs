// https://leetcode.com/problems/lexicographical-numbers/

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = Vec::new();

    fn helper(cur: i32, n: i32, res: &mut Vec<i32>) {
        if cur > n {
            return;
        }
        if cur > 0 {
            res.push(cur);
        }

        for i in 0..=9 {
            if cur * 10 + i == 0 {
                continue;
            }
            helper(cur * 10 + i, n, res);
        }
    }

    helper(0, n, &mut res);

    res
}
