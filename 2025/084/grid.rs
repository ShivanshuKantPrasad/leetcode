// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/description/

pub fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
    fn check_cuts(rectangles: &mut Vec<Vec<i32>>, dim: usize) -> bool {
        let mut gap_count = 0;

        rectangles.sort_unstable_by_key(|x| x[dim]);

        let mut furthest_end = rectangles[0][dim + 2];

        for i in 1..rectangles.len() {
            let rect = &rectangles[i];

            if furthest_end <= rect[dim] {
                gap_count += 1;
            }

            furthest_end = furthest_end.max(rect[dim + 2]);
        }

        gap_count >= 2
    }

    check_cuts(&mut rectangles, 0) || check_cuts(&mut rectangles, 1)
}
