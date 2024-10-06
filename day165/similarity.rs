// https://leetcode.com/problems/sentence-similarity-iii/description/

use std::cmp::Ordering;

pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    // Split sentences into words, stored as slices for zero-copy operations
    let words1: Vec<&str> = sentence1.split_whitespace().collect();
    let words2: Vec<&str> = sentence2.split_whitespace().collect();

    let (shorter, longer) = match words1.len().cmp(&words2.len()) {
        Ordering::Greater => (&words2, &words1),
        _ => (&words1, &words2),
    };

    let shorter_len = shorter.len();
    let longer_len = longer.len();

    // If one sentence is empty, check if the other is also empty
    if shorter_len == 0 {
        return longer_len == 0;
    }

    // Find matching prefix length
    let prefix_len = shorter
        .iter()
        .zip(longer.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // If prefix covers entire shorter sentence, it's similar
    if prefix_len == shorter_len {
        return true;
    }

    // Find matching suffix length
    let suffix_len = shorter
        .iter()
        .rev()
        .zip(longer.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();

    // If suffix covers remaining part after prefix, it's similar
    prefix_len + suffix_len >= shorter_len
}
