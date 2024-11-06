// https://leetcode.com/problems/find-if-array-can-be-sorted/

pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut bits = nums[0].count_ones();
    let mut max = nums[0];
    let mut min = nums[0];

    let mut max_prev = i32::MIN;

    for &num in &nums[1..] {
        if num.count_ones() == bits {
            max = max.max(num);
            min = min.min(num);
        } else {
            if min < max_prev {
                return false;
            }

            max_prev = max;

            max = num;
            min = num;
            bits = num.count_ones();
        }
    }

    min >= max_prev
}
