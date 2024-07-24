// https://leetcode.com/problems/sort-the-jumbled-numbers/description/

pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
    let mut mapped_nums = vec![0; nums.len()];

    for i in 0..nums.len() {
        mapped_nums[i] = String::from_utf8(
            nums[i]
                .to_string()
                .bytes()
                .map(|ch| mapping[(ch - b'0') as usize] as u8 + b'0')
                .collect::<Vec<_>>(),
        )
        .unwrap()
        .parse::<i32>()
        .unwrap();
    }

    let mut num_with_mapped = nums.iter().zip(mapped_nums.iter()).collect::<Vec<_>>();
    num_with_mapped.sort_unstable_by_key(|x| x.1);
    num_with_mapped
        .into_iter()
        .map(|x| *x.0)
        .collect::<Vec<_>>()
}
