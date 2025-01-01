// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/

pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
    folder.sort_unstable();
    let mut result = Vec::new();
    result.push(folder[0].clone());

    folder.iter().skip(1).for_each(|x| {
        if !x.starts_with(&format!("{}/", result.last().unwrap())) {
            result.push(x.clone());
        }
    });

    result
}
