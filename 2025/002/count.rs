// https://leetcode.com/problems/count-vowel-strings-in-ranges/description/

pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let n = words.len();
    let prefix_sum = words
        .iter()
        .map(|word| word.starts_with(&vowels) && word.ends_with(&vowels))
        .enumerate()
        .fold(vec![0; n + 1], |mut res, (index, is_vowel_string)| {
            res[index + 1] = res[index] + if is_vowel_string { 1 } else { 0 };
            res
        });

    queries
        .iter()
        .map(|query| prefix_sum[query[1] as usize + 1] - prefix_sum[query[0] as usize])
        .collect()
}
