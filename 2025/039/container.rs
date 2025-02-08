// https://leetcode.com/problems/design-a-number-container-system/description/

use std::collections::{HashMap, BTreeSet};

struct NumberContainers {
    number: HashMap::<i32, i32>,
    indices: HashMap::<i32, BTreeSet<i32>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            number: HashMap::new(),
            indices: HashMap::new()
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(old_number) = self.number.insert(index, number) {
            self.indices.entry(old_number).or_default().remove(&index);
        }
        self.indices.entry(number).or_default().insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        *self.indices.get(&number).and_then(|indices| indices.first()).unwrap_or(&-1)
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
