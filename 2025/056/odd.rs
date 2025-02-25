// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/submissions/1555089590/

pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
    let MOD = 10i32.pow(9) + 7;
    let n = arr.len();

    for num in &mut arr {
        *num %= 2;
    }

    let mut odd = vec![0; n];
    let mut even = vec![0; n];

    if arr[n - 1] == 0 {
        even[n - 1] = 1;
    } else {
        odd[n - 1] = 1;
    }

    for num in (0..(n - 1)).rev() {
        if arr[num] == 1 {
            odd[num] = (1 + even[num + 1]) % MOD;
            even[num] = odd[num + 1];
        } else {
            even[num] = (1 + even[num + 1]) % MOD;
            odd[num] = odd[num + 1];
        }
    }

    odd.iter().fold(0, |acc, x| (acc + x) % MOD)
}
