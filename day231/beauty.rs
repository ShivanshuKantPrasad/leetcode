// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/description/

pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut ans = 0;
    let mut end = 0;

    for start in 0..nums.len() {
        while end + 1 < nums.len() && nums[end + 1] - nums[start] <= 2 * k {
            end += 1;
        }
        ans = ans.max(end - start + 1);
    }

    ans as i32
}
