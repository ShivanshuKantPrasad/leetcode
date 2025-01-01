// https://leetcode.com/problems/ugly-number-ii/description/

pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut ugly_numbers = vec![0; n];
    ugly_numbers[0] = 1;

    let multiplier = vec![2, 3, 5];
    let mut indices = vec![0, 0, 0];
    let mut multiples = vec![2, 3, 5];

    for i in 1..n {
        let next_ugly_number = *multiples.iter().min().unwrap();
        ugly_numbers[i] = next_ugly_number;

        for j in 0..multiplier.len() {
            if next_ugly_number == multiples[j] {
                indices[j] += 1;
                multiples[j] = ugly_numbers[indices[j]] * multiplier[j];
            }
        }
    }
    ugly_numbers[n - 1]
}
