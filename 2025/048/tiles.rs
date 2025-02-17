// https://leetcode.com/problems/letter-tile-possibilities/description/

pub fn num_tile_possibilities(tiles: String) -> i32 {
    use std::collections::HashSet;

    let n = tiles.len();
    let mut freq = [0; 26];

    for ch in tiles.as_bytes() {
        freq[(ch - b'A') as usize] += 1;
    }

    let mut generated = HashSet::new();

    fn generate(cur: &mut Vec<u8>, generated: &mut HashSet<Vec<u8>>, mut freq: [i32; 26]) {
        for index in 0..26 {
            if freq[index] == 0 {
                continue;
            }
            cur.push(b'A' + index as u8);
            generated.insert(cur.clone());

            freq[index] -= 1;
            generate(cur, generated, freq.clone());

            freq[index] += 1;
            cur.pop();
        }
    }

    generate(&mut Vec::new(), &mut generated, freq);
    generated.len() as i32
}
