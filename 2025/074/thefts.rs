// https://leetcode.com/problems/house-robber-iv/

pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_reward = 1;
    let mut max_reward = *nums.iter().max().unwrap();
    let total_houses = nums.len();

    while min_reward < max_reward {
        let mid_reward = (min_reward + max_reward) / 2;
        let mut possible_thefts = 0;

        let mut i = 0;
        while i < total_houses {
            if nums[i] <= mid_reward {
                possible_thefts += 1;
                i += 1;
            }
            i += 1;
        }

        if possible_thefts >= k {
            max_reward = mid_reward;
        } else {
            min_reward = mid_reward + 1;
        }
    }

    min_reward
}
