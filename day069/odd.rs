pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.windows(3).any(|win| win.iter().all(|x| x % 2 == 1))
}
