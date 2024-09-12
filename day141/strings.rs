// https://leetcode.com/problems/count-the-number-of-consistent-strings/

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let allowed = allowed.chars().collect::<HashSet<_>>();
    words
        .iter()
        .map(|x| x.chars().collect::<HashSet<_>>())
        .filter(|x| x.is_subset(&allowed))
        .count() as i32
}
