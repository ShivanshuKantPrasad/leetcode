// https://leetcode.com/problems/target-sum/description/

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut dp = vec![0; 2 * total_sum as usize + 1];

    dp[(total_sum + nums[0]) as usize] = 1;
    dp[(total_sum - nums[0]) as usize] += 1;

    for index in 1..nums.len() {
        let mut next = vec![0; 2 * total_sum as usize + 1];

        for sum in -total_sum..=total_sum {
            if dp[(sum + total_sum) as usize] > 0 {
                next[(sum + nums[index] + total_sum) as usize] += dp[(sum + total_sum) as usize];
                next[(sum - nums[index] + total_sum) as usize] += dp[(sum + total_sum) as usize];
            }
        }
        dp = next;
    }

    if target.abs() > total_sum {
        0
    } else {
        dp[(target + total_sum) as usize]
    }
}
