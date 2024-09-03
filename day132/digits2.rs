pub fn get_lucky(s: String, k: i32) -> i32 {
    (0..k - 1).fold(
        s.bytes()
            .map(|x| (x - b'a' + 1) as i32)
            .map(|x| if x > 9 { x % 10 + x / 10 } else { x })
            .sum::<i32>(),
        |acc, _| {
            acc.to_string()
                .bytes()
                .map(|x| (x - b'0') as i32)
                .sum::<i32>()
        },
    )
}
