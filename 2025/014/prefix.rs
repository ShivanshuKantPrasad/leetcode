// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/submissions/1508644393/

pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut n = a.len();
    let mut frequency = vec![0; n + 1];

    let mut common_count = 0;
    let mut res = vec![0; a.len()];

    for i in 0..a.len() {
        let a_i = a[i];
        let b_i = b[i];

        frequency[a_i as usize] += 1;
        if frequency[a_i as usize] == 2 {
            common_count += 1;
        }

        frequency[b_i as usize] += 1;
        if frequency[b_i as usize] == 2 {
            common_count += 1;
        }

        res[i] = common_count;
    }

    res
}
