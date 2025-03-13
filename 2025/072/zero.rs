// https://leetcode.com/problems/zero-array-transformation-ii/description/

pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut sum = 0;
    let mut k = 0;

    let mut difference_array = vec![0; n + 1];

    for i in 0..n {
        while (sum + difference_array[i] < nums[i]) {
            k += 1;

            if k > queries.len() {
                return -1;
            }

            let left = queries[k - 1][0] as usize;
            let right = queries[k - 1][1] as usize;
            let val = queries[k - 1][2];

            if right >= i {
                difference_array[left.max(i)] += val;
                difference_array[right + 1] -= val;
            }
        }

        sum += difference_array[i];
    }

    k as i32
}
