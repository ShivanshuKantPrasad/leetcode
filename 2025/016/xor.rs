// https://leetcode.com/problems/bitwise-xor-of-all-pairings/submissions/1510098453/

pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    (if nums1.len() % 2 == 0 {
        0
    } else {
        nums2.iter().fold(0, |acc, num| acc ^ num)
    }) ^ (if nums2.len() % 2 == 0 {
        0
    } else {
        nums1.iter().fold(0, |acc, num| acc ^ num)
    })
}
