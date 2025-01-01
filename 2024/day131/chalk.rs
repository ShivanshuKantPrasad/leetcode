// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/submissions/1375980416/

pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let sum: u64 = chalk.iter().map(|x| *x as u64).sum();
    let mut k = k as u64 % sum;
    for (i, &x) in chalk.iter().enumerate() {
        if k < x as u64 {
            return i as i32;
        }
        k -= x as u64;
    }
    0
}
