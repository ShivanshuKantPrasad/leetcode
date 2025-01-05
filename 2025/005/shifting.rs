pub fn shifting_letters(mut s: String, shifts: Vec<Vec<i32>>) -> String {
    let mut s = s.as_bytes().to_vec();

    let mut num_shifts = vec![0; s.len() + 1];

    for shift in shifts {
        let start = shift[0] as usize;
        let end = shift[1] as usize;
        let direction = if shift[2] == 0 { 25 } else { 1 };

        num_shifts[start] += direction;
        num_shifts[end + 1] += (26 - direction);

        num_shifts[start] %= 26;
        num_shifts[end + 1] %= 26;
    }

    let mut cur_shift = 0;
    for i in 0..s.len() {
        cur_shift += num_shifts[i];
        cur_shift %= 26;
        s[i] = b'a' + (s[i] - b'a' + cur_shift) % 26;
    }

    String::from_utf8(s).unwrap()
}
