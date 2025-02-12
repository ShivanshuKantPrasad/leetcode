// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut nums_with_equal_sum = vec![(i32::MIN, i32::MIN); 100];

    fn sum_digits(mut num: i32) -> usize {
        let mut sum = 0;
        while num != 0 {
            sum += num % 10;
            num /= 10;
        }
        sum as usize
    }

    for num in nums {
        let sum = sum_digits(num);
        let (min, max) = nums_with_equal_sum[sum];

        if num > max {
            nums_with_equal_sum[sum] = (max, num);
        } else if num > min {
            nums_with_equal_sum[sum] = (num, max);
        }
    }

    nums_with_equal_sum
        .into_iter()
        .filter_map(|(min, max)| {
            if min == i32::MIN {
                None
            } else {
                Some(min + max)
            }
        })
        .max()
        .unwrap_or(-1)
}
