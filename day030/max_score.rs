// https://letcode.com/problems/maximum-score-words-formed-by-letters/description/

struct Solution;
impl Solution {
    pub fn is_valid_word(subset_letters: &Vec<i32>, freq: &Vec<i32>) -> bool {
        for c in 0..26 {
            if freq[c] < subset_letters[c] {
                return false;
            }
        }
        return true;
    }

    pub fn check(
        w: i32,
        words: &Vec<String>,
        score: &Vec<i32>,
        subset_letters: &mut Vec<i32>,
        freq: &Vec<i32>,
        total_score: i32,
    ) -> i32 {
        let mut max_score = total_score;
        let mut total_score = total_score;

        if w == -1 {
            return max_score;
        }

        max_score = max_score.max(Self::check(
            w - 1,
            words,
            score,
            subset_letters,
            freq,
            total_score,
        ));

        let l = words[w as usize].len();

        for i in 0..l {
            let c = words[w as usize].as_bytes()[i] - b'a';
            subset_letters[c as usize] += 1;
            total_score += score[c as usize];
        }

        if Self::is_valid_word(subset_letters, freq) {
            max_score = max_score.max(Self::check(
                w - 1,
                words,
                score,
                subset_letters,
                freq,
                total_score,
            ));
        }

        for i in 0..l {
            let c = words[w as usize].as_bytes()[i] - b'a';
            subset_letters[c as usize] -= 1;
            total_score -= score[c as usize];
        }
        return max_score;
    }

    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let w = words.len() as i32;
        let mut freq = vec![0; 26];
        let mut subset_letters = vec![0; 26];

        for c in letters {
            freq[(c as u8 - b'a') as usize] += 1;
        }
        Self::check(w - 1, &words, &score, &mut subset_letters, &freq, 0)
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_score_words(
            vec![
                "dog".to_string(),
                "cat".to_string(),
                "dad".to_string(),
                "good".to_string()
            ],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    );
    println!(
        "{}",
        Solution::max_score_words(
            vec![
                "xxxz".to_string(),
                "ax".to_string(),
                "bx".to_string(),
                "cx".to_string()
            ],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
        )
    );
    println!(
        "{}",
        Solution::max_score_words(
            vec!["leetcode".to_string()],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        )
    );
}
