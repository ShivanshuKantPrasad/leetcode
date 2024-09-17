// https://leetcode.com/problems/uncommon-words-from-two-sentences/

use std::collections::HashMap;
pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let count =
        s1.split(' ')
            .chain(s2.split(' '))
            .fold(HashMap::<&str, i32>::new(), |mut map, word| {
                *map.entry(word).or_default() += 1;
                map
            });
    count
        .into_iter()
        .filter(|x| x.1 == 1)
        .map(|x| x.0.to_string())
        .collect::<Vec<_>>()
}
