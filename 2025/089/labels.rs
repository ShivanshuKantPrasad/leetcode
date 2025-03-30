// https://leetcode.com/problems/partition-labels/description/

pub fn partition_labels(s: String) -> Vec<i32> {
    let mut last_occurence = vec![0; 26];

    let s = s.as_bytes();

    for (i, &ch) in s.iter().enumerate() {
        last_occurence[(ch - b'a') as usize] = i;
    }

    let mut partition_end = 0;
    let mut partition_start = 0;
    let mut partition_sizes = vec![];

    for i in 0..s.len() {
        partition_end = partition_end.max(last_occurence[(s[i] - b'a') as usize]);
        if i == partition_end {
            partition_sizes.push((i - partition_start + 1) as i32);
            partition_start = i + 1;
        }
    }

    partition_sizes
}
