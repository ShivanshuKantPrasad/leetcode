pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    if !expression.contains(&['+', '-', '*']) {
        return vec![expression.parse::<i32>().unwrap()];
    }

    let mut result = Vec::new();

    for (i, op) in expression.match_indices(&['+', '-', '*']) {
        let left_values = diff_ways_to_compute(expression[..i].into());
        let right_values = diff_ways_to_compute(expression[i + 1..].into());
        for lhs in left_values {
            for &rhs in &right_values {
                match op {
                    "+" => result.push(lhs + rhs),
                    "-" => result.push(lhs - rhs),
                    "*" => result.push(lhs * rhs),
                    _ => unreachable!(),
                }
            }
        }
    }
    result
}
