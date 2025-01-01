// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/description

pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();
    let n1 = str1.len();
    let n2 = str2.len();

    let mut i = 0;
    let mut j = 0;
    while i < n1 && j < n2 {
        if str1[i] == str2[j] || str1[i] + 1 == str2[j] || str1[i] - 25 == str2[j] {
            j += 1;
        }
        i += 1;
    }
    j == n2
}
