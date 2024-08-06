// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/

pub fn minimum_pushes(word: String) -> i32 {
    let mut freq = vec![0; 26];
    let word = word.as_bytes();
    for ch in word {
        freq[(ch - b'a') as usize] += 1;
    }

    freq.sort();
    freq.reverse();
    freq.iter()
        .enumerate()
        .fold(0, |acc, (i, fq)| acc + fq * (i / 8 + 1)) as i32
}
