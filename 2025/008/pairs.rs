// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/description/

pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            words
                .iter()
                .skip(i + 1)
                .filter(|x| x.starts_with(word) && x.ends_with(word))
                .count()
        })
        .sum::<usize>() as i32
}
