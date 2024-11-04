// https://leetcode.com/problems/string-compression-iii/

pub fn compressed_string(word: String) -> String {
    let mut word = word.chars().peekable();
    let mut res = String::new();

    let mut prev = ' ';
    let mut count = 0;

    while let Some(ch) = word.next() {
        match ch {
            x if x == prev => {
                if count < 9 {
                    count += 1;
                } else {
                    res += &format!("9{ch}");
                    count = 1;
                }
            }
            _ => {
                if count != 0 {
                    res += &format!("{count}{prev}");
                }
                prev = ch;
                count = 1;
            }
        }
    }

    res += &format!("{count}{prev}");

    res
}
