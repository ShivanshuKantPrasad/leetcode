// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/description/

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    fn min_swaps_helper(nums: &Vec<i32>, val: i32) -> i32 {
        let length = nums.len();

        let total = nums.iter().filter(|x| **x == val).count();

        if total == 0 || total == length {
            return 0;
        }
        let mut start = 0;
        let mut end = total;
        let mut max_val_in_window = nums[start..end].iter().filter(|x| **x == val).count();
        let mut cur_val_in_window = max_val_in_window;

        while end < length {
            if nums[start] == val {
                cur_val_in_window -= 1;
            }
            if nums[end] == val {
                cur_val_in_window += 1;
            }
            start += 1;
            end += 1;
            max_val_in_window = max_val_in_window.max(cur_val_in_window);
        }

        return (total - max_val_in_window) as i32;
    }

    let op1 = min_swaps_helper(&nums, 0);
    let op2 = min_swaps_helper(&nums, 1);
    op1.min(op2)
}
