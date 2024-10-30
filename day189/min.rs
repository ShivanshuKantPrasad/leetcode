// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/submissions/1438294162/

pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let N = nums.len();

    let mut lis_length = vec![1; N];
    let mut lds_length = vec![1; N];

    for i in 0..N {
        for j in (0..i).rev() {
            if nums[i] > nums[j] {
                lis_length[i] = lis_length[i].max(lis_length[j] + 1);
            }
        }
    }

    for i in (0..N).rev() {
        for j in (i + 1)..N {
            if nums[i] > nums[j] {
                lds_length[i] = lds_length[i].max(lds_length[j] + 1);
            }
        }
    }

    let mut min = i32::MAX;

    for i in 0..N {
        if lis_length[i] > 1 && lds_length[i] > 1 {
            min = min.min(N as i32 - lis_length[i] - lds_length[i] + 1);
        }
    }

    min
}
