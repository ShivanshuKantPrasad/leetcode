// https://leetcode.com/problems/relative-sort-array/

struct Solution;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let pos = arr2
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, i as i32))
            .collect::<std::collections::HashMap<i32, i32>>();
        let mut res = arr1;
        res.sort_by_key(|x| *pos.get(&x).unwrap_or(&(x + 1500)));
        res
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        )
    );
    println!(
        "{:?}",
        Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
    );
}
