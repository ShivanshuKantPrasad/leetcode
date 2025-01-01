// https://leetcode.com/problems/find-k-th-smallest-pair-distance/

pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    fn count_pairs_with_max_distance(nums: &Vec<i32>, max_distance: i32) -> i32 {
        let mut count = 0;
        let n = nums.len();
        let mut left = 0;

        for right in 0..n {
            while nums[right] - nums[left] > max_distance {
                left += 1;
            }
            count += right - left;
        }
        count as i32
    }

    nums.sort_unstable();

    let mut low = 0;
    let mut high = nums[nums.len() - 1] - nums[0];

    while low < high {
        let mid = (low + high) / 2;

        let count = count_pairs_with_max_distance(&nums, mid);

        if count < k {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}
