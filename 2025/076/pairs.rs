// https://leetcode.com/problems/divide-array-into-equal-pairs/description/

pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut n = nums.len();

    let mut count = vec![0; 501];

    for num in nums {
        count[num as usize] += 1;
    }

    count.iter().all(|x| x % 2 == 0)
}
