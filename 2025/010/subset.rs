// https://leetcode.com/problems/word-subsets/submissions/1503869079/

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    fn counter(word: &str) -> Vec<i32> {
        word.as_bytes()
            .iter()
            .fold(vec![0; 26], |mut count, letter| {
                count[(letter - b'a') as usize] += 1;
                count
            })
    }

    let max_counter = words2.iter().fold(vec![0; 26], |mut max_count, word| {
        let count = counter(&word);
        for i in 0..26 {
            max_count[i] = max_count[i].max(count[i]);
        }
        max_count
    });

    words1
        .into_iter()
        .filter(|word| {
            let word_count = counter(word);
            word_count.iter().zip(&max_counter).all(|(a, b)| a >= b)
        })
        .collect()
}
