// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/submissions/1602489592/

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut st = HashSet::new();
    for x in nums {
        if x < k {
            return -1;
        } else if x > k {
            st.insert(x);
        }
    }
    st.len() as i32
}
