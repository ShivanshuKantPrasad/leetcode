// https://leetcode.com/problems/intersection-of-two-arrays-ii/

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let freq1 = nums1.into_iter().fold([0; 1001], |mut arr, i| {
        arr[i as usize] += 1;
        arr
    });
    let freq2 = nums2.into_iter().fold([0; 1001], |mut arr, i| {
        arr[i as usize] += 1;
        arr
    });
    let intersection = (0..1001).into_iter().fold([0; 1001], |mut arr, i| {
        arr[i] = freq1[i].min(freq2[i]);
        arr
    });

    let mut result = vec![];
    for i in 0..1001 {
        let freq = intersection[i];
        for _ in 0..freq {
            result.push(i as i32);
        }
    }
    result
}
