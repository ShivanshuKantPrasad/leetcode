// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description/

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split_whitespace()
        .position(|word| word.starts_with(&search_word))
        .map(|i| i as i32 + 1)
        .unwrap_or(-1)
}
