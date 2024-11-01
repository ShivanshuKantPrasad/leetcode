// https://leetcode.com/problems/delete-characters-to-make-fancy-string/description/

pub fn make_fancy_string(s: String) -> String {
    let mut last_char = ' ';
    let mut chain = 0;
    let mut remaining_chars = Vec::new();

    for ch in s.chars() {
        if ch == last_char {
            if chain != 2 {
                chain += 1;
                remaining_chars.push(last_char);
            }
        } else {
            chain = 1;
            last_char = ch;
            remaining_chars.push(last_char);
        }
    }

    remaining_chars.iter().collect()
}
