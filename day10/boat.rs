struct Solution;
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut start = 0;
        let mut end = people.len() - 1;
        let mut boats = 0;
        while (end > start) {
            if (start != end && limit - people[end] >= people[start]) {
                start += 1;
            }
            end -= 1;
            boats += 1;
        }
        boats + if start == end { 1 } else { 0 }
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![1, 2], 3));
    println!("{}", Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
    println!("{}", Solution::num_rescue_boats(vec![3, 5, 3, 4], 5));
}
