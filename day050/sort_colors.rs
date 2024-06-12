// https://leetcode.com/problems/sort-colors/?envType=daily-question&envId=2024-06-11

struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut mid = 0;

        while mid <= high {
            if nums[mid] == 0 {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            } else if nums[mid] == 1 {
                mid += 1;
            } else if nums[mid] == 2 {
                nums.swap(mid, high);
                if high == 0 {
                    break;
                }
                high -= 1;
            }
        }
    }
}

fn main() {
    let mut arr = vec![2, 0, 2, 1, 1, 0];
    println!("Original Array: {arr:?}");
    Solution::sort_colors(&mut arr);
    println!("Sorted Array: {arr:?}");

    let mut arr = vec![2, 0, 1];
    println!("Original Array: {arr:?}");
    Solution::sort_colors(&mut arr);
    println!("Sorted Array: {arr:?}");
}
