// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/description/

pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut right = arr.len() - 1;
    while right > 0 && arr[right] >= arr[right - 1] {
        right -= 1;
    }

    let mut ans = right;
    let mut left = 0;

    while left < right && (left == 0 || arr[left - 1] <= arr[left]) {
        while right < arr.len() && arr[left] > arr[right] {
            right += 1;
        }
        ans = ans.min(right - left - 1);
        left += 1;
    }
    ans as i32
}
