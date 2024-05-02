struct Solution;
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut sol = nums.clone();
        sol.sort();
        let mut i = 0;
        let mut j = sol.len() - 1;
        while (sol[i] != -sol[j] && i < j) {
            println!("{} {}", sol[i], sol[j]);
            if (sol[i].abs() < sol[j]) {
                j -= 1;
            } else {
                i += 1;
            }
        }
        if (i < j) {
            return sol[j];
        } else {
            return -1;
        }
    }
}

fn main() {
    println!("{}", Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]));
    println!("{}", Solution::find_max_k(vec![-1, 2, -3, 3]));
    println!("{}", Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]));
    println!(
        "{}",
        Solution::find_max_k(vec![14, 33, 40, -33, 8, -24, -42, 30, -18, -34])
    );
}
