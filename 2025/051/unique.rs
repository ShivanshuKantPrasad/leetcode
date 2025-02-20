// https://leetcode.com/problems/find-unique-binary-string/

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let mut result = Vec::<char>::with_capacity(n);

    for i in 0..n {
        result.push(if nums[i].chars().nth(i).unwrap() == '0' {
            '1'
        } else {
            '0'
        });
    }

    result.into_iter().collect()
}
