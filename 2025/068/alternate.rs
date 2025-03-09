// https://leetcode.com/problems/alternating-groups-ii/description/

pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
    colors.extend_from_within(0..(k - 1) as usize);

    let mut alternate = vec![1; colors.len()];
    for i in 1..colors.len() {
        alternate[i] = if colors[i - 1] != colors[i] {
            alternate[i - 1] + 1
        } else {
            1
        }
    }
    alternate.iter().filter(|x| **x >= k).count() as i32
}
