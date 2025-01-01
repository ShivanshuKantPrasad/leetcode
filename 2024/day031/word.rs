// https://leetcode.com/problems/word-break-ii/description/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dp = HashMap::<i32, Vec<String>>::new();

        for start_id in (0..s.len()).rev() {
            let mut valid_sentences = Vec::<String>::new();

            for end_id in start_id..s.len() {
                let current_word = String::from(&s[start_id..end_id + 1]);

                if word_dict.contains(&current_word) {
                    if end_id == s.len() - 1 {
                        valid_sentences.push(current_word);
                    } else {
                        let sentences_from_next_index = &dp[&(end_id as i32 + 1)];

                        for sentence in sentences_from_next_index {
                            valid_sentences.push(current_word.clone() + " " + sentence.as_str());
                        }
                    }
                }
            }

            dp.insert(start_id as i32, valid_sentences);
        }

        dp.entry(0).or_default().clone()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string()
            ]
        )
    );
    println!(
        "{:?}",
        Solution::word_break(
            "pineapplepenapple".to_string(),
            vec![
                "apple".to_string(),
                "pen".to_string(),
                "applepen".to_string(),
                "pine".to_string(),
                "pineapple".to_string()
            ]
        )
    );
    println!(
        "{:?}",
        Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        )
    );
}
