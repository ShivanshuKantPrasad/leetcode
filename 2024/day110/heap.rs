// https://leetcode.com/problems/kth-largest-element-in-a-stream/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    size: usize,
    heap: BinaryHeap<Reverse<i32>>,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let size = k as usize;
        let mut heap = BinaryHeap::with_capacity(size + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > size {
                heap.pop();
            }
        }
        Self {
            size,
            heap,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.size {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
