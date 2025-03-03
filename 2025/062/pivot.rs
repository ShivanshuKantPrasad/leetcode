// https://leetcode.com/problems/partition-array-according-to-given-pivot/description/

pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let n = nums.len();

    let mut res = vec![0; n];
    let mut less_i = 0;
    let mut greater_i = n - 1;
    for i in 0..n {
        if nums[i] < pivot {
            res[less_i] = nums[i];
            less_i += 1;
        }
        let j = n - i - 1;
        if nums[j] > pivot {
            res[greater_i] = nums[j];
            greater_i -= 1;
        }
    }

    while less_i <= greater_i {
        res[less_i] = pivot;
        less_i += 1;
    }

    res
}
