// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/description/

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;

    let n = blocks.len();
    let blocks = blocks.as_bytes();

    let mut white_count = vec![0; n + 1];

    for (i, &b) in blocks.iter().enumerate() {
        white_count[i + 1] = white_count[i] + if b == b'W' { 1 } else { 0 };
    }

    let mut min = i32::MAX;
    for i in k..=n {
        let recolors = white_count[i] - white_count[i - k];
        min = min.min(recolors);
    }

    min
}
