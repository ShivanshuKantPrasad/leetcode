// https://leetcode.com/problems/my-calendar-i/description/

#[derive(Default)]
struct MyCalendar {
    events: std::collections::BTreeSet<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Default::default()
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        self.events.iter().all(|&(s, e)| end <= s || start >= e) && self.events.insert((start, end))
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
