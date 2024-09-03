//https://leetcode.com/problems/sum-of-digits-of-string-after-convert/description/

pub fn get_lucky(s: String, k: i32) -> i32 {
    let s = s
        .bytes()
        .map(|x| (x - b'a' + 1).to_string())
        .collect::<String>();
    (0..k)
        .fold((0, s), |acc, _| {
            let res = acc.1.bytes().map(|x| (x - b'0') as i32).sum::<i32>();
            let s = res.to_string();
            (res, s)
        })
        .0
}
