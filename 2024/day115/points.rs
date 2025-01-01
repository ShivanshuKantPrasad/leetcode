// https://leetcode.com/problems/maximum-number-of-points-with-cost/description/

pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
    let cols = points[0].len();
    let mut current_row = vec![0; cols];
    let mut previous_row = vec![0; cols];

    for row in points {
        let mut running_max = 0;

        for i in 0..cols {
            running_max = i64::max(running_max - 1, previous_row[i]);
            current_row[i] = running_max;
        }

        running_max = 0;
        for i in (0..cols).rev() {
            running_max = i64::max(running_max - 1, previous_row[i]);
            current_row[i] = current_row[i].max(running_max) + row[i] as i64;
        }

        previous_row = current_row.clone();
    }

    *previous_row.iter().max().unwrap()
}
