// https://leetcode.com/problems/string-matching-in-an-array/description/

pub fn string_matching(words: Vec<String>) -> Vec<String> {
    words
        .iter()
        .filter(|word| {
            words
                .iter()
                .filter(|x| x != word)
                .any(|x| x.contains(*word))
        })
        .map(|x| x.to_string())
        .collect()
}
