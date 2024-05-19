// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/

struct Solution;
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut count = 0;
        let mut positive_minimum = (1 << 30);
        let mut negative_maximum = -1 * (1 << 30);
        let k = k as i64;

        for node in nums {
            let node = node as i64;
            let operated_node_value = node ^ k;
            sum += node;
            let net_change = operated_node_value - node;

            if net_change > 0 {
                positive_minimum = positive_minimum.min(net_change);
                sum += net_change;
                count += 1;
            } else {
                negative_maximum = negative_maximum.max(net_change);
            }
        }

        if count % 2 == 0 {
            return sum;
        }
        return i64::max(sum - positive_minimum, sum + negative_maximum);
    }
}

fn main() {
    println!(
        "{}",
        Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]])
    );
    println!(
        "{}",
        Solution::maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]])
    );
    println!(
        "{}",
        Solution::maximum_value_sum(
            vec![7, 7, 7, 7, 7, 7],
            3,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
        )
    );
}
