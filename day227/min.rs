// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/description/

pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    fn is_possible(max_balls_in_bag: i32, nums: &Vec<i32>, max: i32) -> bool {
        let mut total = 0;
        for &num in nums {
            let operations = (num as f64 / max_balls_in_bag as f64).ceil() as i32 - 1;
            total += operations;
        }
        if total > max {
            return false;
        }
        true
    }

    let mut left = 1;
    let mut right = 0;

    for &num in &nums {
        right = right.max(num);
    }

    while left < right {
        let middle = (left + right) / 2;

        if is_possible(middle, &nums, max_operations) {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    left
}
