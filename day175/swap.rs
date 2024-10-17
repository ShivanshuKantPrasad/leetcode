// https://leetcode.com/problems/maximum-swap/

pub fn maximum_swap(num: i32) -> i32 {
    let mut digits = num
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let n = digits.len();

    let mut max_right_index = vec![0; n];
    max_right_index[n - 1] = n - 1;

    for i in (0..n - 1).rev() {
        max_right_index[i] = if digits[i] > digits[max_right_index[i + 1]] {
            i
        } else {
            max_right_index[i + 1]
        };
    }

    for i in 0..n {
        if digits[i] < digits[max_right_index[i]] {
            digits.swap(i, max_right_index[i]);
            return digits.into_iter().fold(0, |acc, digit| 10 * acc + digit);
        }
    }

    num
}
