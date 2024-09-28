// https://leetcode.com/problems/design-circular-deque/submissions/1404632171/

struct MyCircularDeque {
    v: Vec<i32>,
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            v: vec![-1; k],
            front: 0,
            back: k - 1,
            size: 0,
            capacity: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.v[self.front] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.back = (self.back + 1) % self.capacity;
        self.v[self.back] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.back = (self.back + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.v[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.v[self.back]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}
