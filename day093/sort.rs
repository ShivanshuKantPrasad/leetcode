// https://leetcode.com/problems/sort-an-array/

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    fn merge(arr: &mut [i32], mid: usize) {
        let mut i = 0;
        let mut j = mid;

        while i < j && j < arr.len() {
            if arr[i] <= arr[j] {
                i += 1;
            } else {
                let mut k = j;
                while k > i {
                    arr.swap(k, k - 1);
                    k -= 1;
                }
                i += 1;
                j += 1;
            }
        }
    }

    fn merge_sort(arr: &mut [i32]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        let mid = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
    merge_sort(&mut nums);
    nums
}
