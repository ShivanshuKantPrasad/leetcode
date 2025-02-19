// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/description/

pub fn get_happy_string(n: i32, k: i32) -> String {
    let mut happy = Vec::new();

    fn generate(cur: String, remaining: i32, k: i32, happy: &mut Vec<String>) {
        if remaining == 0 {
            return;
        }

        let prev = cur.chars().last().unwrap_or('d');

        for ch in ['a', 'b', 'c'] {
            if ch != prev {
                let new = cur.clone() + &ch.to_string();
                if remaining == 1 {
                    happy.push(new.clone());
                }
                generate(new, remaining - 1, k, happy);
            }
            if k as usize == happy.len() {
                return;
            }
        }
    }

    generate(String::new(), n, k, &mut happy);
    happy.get(k as usize - 1).unwrap_or(&String::new()).clone()
}
