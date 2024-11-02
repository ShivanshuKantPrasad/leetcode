// https://leetcode.com/problems/circular-sentence/

pub fn is_circular_sentence(sentence: String) -> bool {
    let words = sentence.split_whitespace();
    words
        .clone()
        .zip(words.cycle().skip(1))
        .all(|(a, b)| a.chars().last() == b.chars().next())
}
