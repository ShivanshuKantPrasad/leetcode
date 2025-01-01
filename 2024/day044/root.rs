// https://leetcode.com/problems/replace-words/submissions/1280293274/

struct Solution;
impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        dictionary.sort_by_key(|x| x.len());

        sentence
            .split(" ")
            .map(|x| {
                dictionary
                    .iter()
                    .find(|root| x.starts_with(*root))
                    .cloned()
                    .unwrap_or(x.to_string())
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    println!(
        "{}",
        Solution::replace_words(
            vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
            "the cattle was rattled by the battery".to_string()
        )
    );
    println!(
        "{}",
        Solution::replace_words(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "aadsfasf absbs bbab cadsfafs".to_string()
        )
    );
}
