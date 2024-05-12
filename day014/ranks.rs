// https://leetcode.com/problems/relative-ranks/description/

struct Solution;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.clone();
        sorted.sort_unstable();
        let n = sorted.len();

        score
            .iter()
            .map(|i| n - sorted.binary_search(i).unwrap())
            .map(|x| match x {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                x => x.to_string(),
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec![
            "Gold Medal".to_string(),
            "Silver Medal".to_string(),
            "Bronze Medal".to_string(),
            "4".to_string(),
            "5".to_string()
        ]
    );
    println!("{:?}", Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]));
    println!("{:?}", Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]));
}
