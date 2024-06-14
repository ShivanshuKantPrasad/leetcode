// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/description/

struct Solution;
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let (mut seats, mut students) = (seats, students);
        seats.sort();
        students.sort();
        seats
            .iter()
            .zip(students.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4])
    );
    println!(
        "{}",
        Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6])
    );
    println!(
        "{}",
        Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6])
    );
}
