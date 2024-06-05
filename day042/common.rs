// https://leetcode.com/problems/find-common-characters/description/

struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut freq = vec![vec![0; words.len()]; 26];
        for (i, word) in words.iter().enumerate() {
            for ch in word.as_bytes() {
                freq[(ch - b'a') as usize][i] += 1;
            }
        }

        freq.iter()
            .enumerate()
            .map(|(i, word_freq)| {
                vec![
                    String::from((i as u8 + b'a') as char);
                    *word_freq.iter().min().unwrap() as usize
                ]
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::common_chars(vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string()
        ])
    );
    println!(
        "{:?}",
        Solution::common_chars(vec![
            "cool".to_string(),
            "lock".to_string(),
            "cook".to_string()
        ])
    );
}
