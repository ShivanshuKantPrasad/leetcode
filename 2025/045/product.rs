// https://leetcode.com/problems/product-of-the-last-k-numbers/description/

struct ProductOfNumbers {
    products: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            products: vec![1]
        }
    }
    
    fn add(&mut self, num: i32) {
        if (num == 0) {
            self.products = vec![1];
        }
        else {
            self.products.push(num * self.products.last().unwrap());
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let size = self.products.len() - 1;
        if (k > size) { return 0; }
        return self.products[size] / self.products[size - k];
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
