// https://leetcode.com/problems/sort-the-people/submissions/1328978792/

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut names = names.into_iter().zip(heights).collect::<Vec<_>>();
    names.sort_unstable_by_key(|name| name.1);
    names.into_iter().map(|name| name.0).rev().collect()
}
