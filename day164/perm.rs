// https://leetcode.com/problems/permutation-in-string/description/

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut freq = [0; 26];

    s1.bytes().for_each(|ch| freq[(ch - b'a') as usize] += 1);

    let n = s1.len();
    let s2 = s2.as_bytes();

    s2[0..n]
        .iter()
        .for_each(|ch| freq[(ch - b'a') as usize] -= 1);

    if freq.iter().all(|e| *e == 0) {
        return true;
    }

    for l in 1..=(s2.len() - n) {
        freq[(s2[l - 1] - b'a') as usize] += 1;
        freq[(s2[l + n - 1] - b'a') as usize] -= 1;
        if freq.iter().all(|e| *e == 0) {
            return true;
        }
    }

    false
}
