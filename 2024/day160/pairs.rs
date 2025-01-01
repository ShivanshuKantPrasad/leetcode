// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/description/

pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut freq = vec![0; k];

    for num in arr {
        freq[num.rem_euclid(k as i32) as usize] += 1;
    }

    let mid = k / 2;
    (1..=mid).all(|i| freq[i] == freq[k - i]) && (k % 2 != 0 || freq[mid] % 2 == 0)
}
