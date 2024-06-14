// https://leetcode.com/problems/minimum-increment-to-make-array-unique/

struct Solution;
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 100050];
        let mut result = 0;
        for num in nums {
            freq[num as usize] += 1;
        }

        for i in 0..freq.len() {
            if freq[i] > 1 {
                let x = freq[i] - 1;
                result += x;
                if i == freq.len() - 1 {
                    result += (x * (x - 1)) / 2
                } else {
                    freq[i + 1] += x;
                }
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::min_increment_for_unique(vec![1, 2, 2]));
    println!(
        "{}",
        Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7])
    );
}
