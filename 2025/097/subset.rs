// https://leetcode.com/problems/partition-equal-subset-sum/description/

    pub fn can_partition(nums: Vec<i32>) -> bool {
        
        let n = nums.len();
        let sum = nums.iter().sum::<i32>();

        if sum % 2 != 0 { return false; }

        let target_sum = (sum / 2) as usize;

        let mut dp = vec![false; target_sum + 1];
        dp[0] = true;

        for num in nums {
            let num = num as usize;
            for curr_sum in (num..=target_sum).rev() {
                dp[curr_sum] = dp[curr_sum] || dp[curr_sum - num];
            }
            if dp[target_sum] { return true; }
        }

        dp[target_sum]
    }
