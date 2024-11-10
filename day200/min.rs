// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/submissions/1448465500/

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    fn update_bit_counts(bit_counts: &mut Vec<i32>, number: i32, delta: i32) {
        for bit_position in 0..32 {
            if (number >> bit_position) & 1 == 1 {
                bit_counts[bit_position] += delta;
            }
        }
    }

    fn convert_bit_counts_to_number(bit_counts: &Vec<i32>) -> i32 {
        bit_counts
            .iter()
            .enumerate()
            .fold(0, |res, (bit_position, bit_count)| {
                res | if *bit_count != 0 {
                    1 << bit_position
                } else {
                    0
                }
            })
    }

    let mut min_length = usize::MAX;
    let mut window_start = 0;
    let mut window_end = 0;
    let mut bit_counts = vec![0; 32];

    while window_end < nums.len() {
        update_bit_counts(&mut bit_counts, nums[window_end], 1);

        while window_start <= window_end && convert_bit_counts_to_number(&bit_counts) >= k {
            min_length = min_length.min(window_end - window_start + 1);
            update_bit_counts(&mut bit_counts, nums[window_start], -1);
            window_start += 1;
        }
        window_end += 1;
    }

    if min_length == usize::MAX {
        -1
    } else {
        min_length as i32
    }
}
