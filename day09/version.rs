struct Solution;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1
            .split('.')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut v2 = version2
            .split('.')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let n1 = v1.len();
        let n2 = v2.len();
        let n = n1.max(n2);
        v1.resize(n, 0);
        v2.resize(n, 0);

        for i in 0..n {
            if v1[i] > v2[i] {
                return 1;
            } else if v1[i] < v2[i] {
                return -1;
            }
        }
        return 0;
    }
}

fn main() {
    println!(
        "{}",
        Solution::compare_version("1.01".to_string(), "1.001".to_string())
    );
    println!(
        "{}",
        Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
    );
    println!(
        "{}",
        Solution::compare_version("0.1".to_string(), "1.1".to_string())
    );
}
