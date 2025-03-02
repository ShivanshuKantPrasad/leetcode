// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/description/

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n1 = nums1.len();
    let n2 = nums2.len();
    let mut result = Vec::new();

    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < n1 && i2 < n2 {
        if nums1[i1][0] < nums2[i2][0] {
            result.push(nums1[i1].clone());
            i1 += 1;
        } else if nums1[i1][0] > nums2[i2][0] {
            result.push(nums2[i2].clone());
            i2 += 1;
        } else {
            result.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
            i1 += 1;
            i2 += 1;
        }
    }

    while i1 < n1 {
        result.push(nums1[i1].clone());
        i1 += 1;
    }

    while i2 < n2 {
        result.push(nums2[i2].clone());
        i2 += 1;
    }

    result
}
