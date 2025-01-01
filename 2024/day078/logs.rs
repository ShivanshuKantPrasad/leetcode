// https://leetcode.com/problems/crawler-log-folder/description/

pub fn min_operations(logs: Vec<String>) -> i32 {
    logs.into_iter().fold(0, |acc, log| match log.as_str() {
        "../" => 0.max(acc - 1),
        "./" => acc,
        _ => acc + 1,
    })
}

fn main() {
    println!(
        "{}",
        min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string()
        ]),
    );
    println!(
        "{}",
        min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string()
        ]),
    );
    println!(
        "{}",
        min_operations(vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string()
        ])
    );
}
