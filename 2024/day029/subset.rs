// https://leetcode.com/problems/the-number-of-beautiful-subsets/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 1;
        let mut freq_map: HashMap<i32, HashMap<i32, i32>> = Default::default();

        for num in nums {
            *freq_map.entry(num % k).or_default().entry(num).or_default() += 1;
        }
        let mut freq_map = freq_map
            .iter()
            .map(|(key, value)| {
                (
                    *key,
                    value
                        .iter()
                        .map(|(x, y)| (*x, *y))
                        .collect::<Vec<(i32, i32)>>(),
                )
            })
            .collect::<HashMap<i32, Vec<(i32, i32)>>>();

        for (_, value) in &mut freq_map {
            value.sort();
        }

        for fq in freq_map {
            let mut prev_num = -k;
            let mut prev2 = 1;
            let mut prev1 = 1;
            let mut curr = 0;

            for (num, freq) in fq.1 {
                let skip = prev1;

                let take;

                if num - prev_num == k {
                    take = ((1 << freq) - 1) * prev2;
                } else {
                    take = ((1 << freq) - 1) * prev1;
                }

                curr = skip + take;
                prev2 = prev1;
                prev1 = curr;
                prev_num = num;
            }
            res *= curr;
        }
        res - 1
    }
}

fn main() {
    println!("{}", Solution::beautiful_subsets(vec![2, 4, 6], 2));
    println!("{}", Solution::beautiful_subsets(vec![1], 1));
}
