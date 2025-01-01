// https://leetcode.com/problems/design-a-stack-with-increment-operation/submissions/1406733356/

struct CustomStack {
    arr: Vec<i32>,
    size: usize

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack {
            arr: Vec::with_capacity(maxSize as usize),
            size: maxSize as usize
        }
        
    }
    
    fn push(&mut self, x: i32) {
        if self.arr.len() == self.size { return; }
        self.arr.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.arr.pop().unwrap_or(-1)
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        if k < 0 { return; }
        let k = k as usize;
        for i in 0..k.min(self.arr.len()) {
            self.arr[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
