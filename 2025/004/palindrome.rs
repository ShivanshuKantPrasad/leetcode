// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description/

pub fn count_palindromic_subsequence(s: String) -> i32 {
    use std::collections::HashSet;
    let s = s.as_bytes();
    let mut letters = s.iter().collect::<HashSet<_>>();
    println!("{letters:?}");

    let mut ans = 0;
    for letter in letters {
        let mut i = -1;
        let mut j = 0;

        for k in 0..s.len() {
            if s[k] == *letter {
                if i == -1 {
                    i = k as i32;
                }
                j = k as i32;
            }
        }

        let mut between = HashSet::new();
        for k in ((i + 1) as usize)..(j as usize) {
            between.insert(s[k]);
        }
        // let between = s
        //     .iter()
        //     .skip((i + 1) as usize)
        //     .take(((j as usize).saturating_sub(i as usize).saturating_sub(1)))
        //     .collect::<HashSet<_>>();
        ans += between.len() as i32;
    }
    ans
}
