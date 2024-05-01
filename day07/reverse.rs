struct Solution;
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut vec: Vec<String> = word.split_inclusive(ch).map(|x| x.to_string()).collect();
        if vec.len() > 1 {
            vec[0] = vec[0].chars().rev().collect();
        }

        match word.find(ch) {
            None => word,
            Some(x) => {
                let (a, b) = word.split_at(x + 1);
                a.chars().rev().collect::<String>() + b
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::reverse_prefix("abcdefd".to_string(), 'd'));
}
