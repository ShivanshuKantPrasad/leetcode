pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut i = 0;
    let mut longest = 0;
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    for j in 0..nums.len() {
        max_heap.push((nums[j], j));
        min_heap.push(Reverse((nums[j], j)));

        let mut max_value = max_heap.peek().unwrap().0;
        let mut min_value = min_heap.peek().unwrap().0 .0;
        let mut diff = max_value - min_value;
        while diff > limit {
            i = max_heap
                .peek()
                .unwrap()
                .1
                .min(min_heap.peek().unwrap().0 .1)
                + 1;

            while max_heap.peek().is_some() && max_heap.peek().unwrap().1 < i {
                max_heap.pop();
            }
            while min_heap.peek().is_some() && min_heap.peek().unwrap().0 .1 < i {
                min_heap.pop();
            }
            max_value = max_heap.peek().unwrap().0;
            min_value = min_heap.peek().unwrap().0 .0;
            diff = max_value - min_value;
        }
        longest = longest.max(j - i + 1);
    }
    longest as i32
}
fn main() {
    println!("{}", longest_subarray(vec![8, 2, 4, 7], 4));
    println!("{}", longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
    println!("{}", longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0));
    println!(
        "{}",
        longest_subarray(vec![2, 2, 2, 4, 4, 2, 5, 5, 5, 5, 5, 2], 2)
    );
}
