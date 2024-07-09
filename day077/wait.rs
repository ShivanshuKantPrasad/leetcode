// https://leetcode.com/problems/average-waiting-time/

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    customers
        .iter()
        .scan(0, |current, customer| {
            *current = i32::max(*current, customer[0]) + customer[1];
            Some((*current - customer[0]) as f64 / customers.len() as f64)
        })
        .sum()
}

fn main() {
    println!(
        "{}",
        average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]])
    );
    println!(
        "{}",
        average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]])
    );
}
