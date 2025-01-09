// https://leetcode.com/problems/counting-words-with-a-given-prefix/description/

pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|word| word.starts_with(&pref)).count() as i32
}
