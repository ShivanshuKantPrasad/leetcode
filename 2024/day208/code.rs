// https://leetcode.com/problems/defuse-the-bomb/

pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    match k {
        0 => {
            vec![0; n]
        }
        i32::MIN..=-1 => {
            let mut res = vec![0; n];
            for i in 0..n {
                for j in 1..=(k.abs() as usize) {
                    res[i] += code[(n + i - j) % n];
                }
            }
            res
        }
        1..=i32::MAX => code
            .clone()
            .into_iter()
            .chain(code.into_iter())
            .collect::<Vec<_>>()
            .windows(k as usize)
            .skip(1)
            .take(n)
            .map(|w| w.into_iter().sum())
            .collect(),
    }
}
