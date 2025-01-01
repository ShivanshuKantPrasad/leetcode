// https://leetcode.com/problems/ipo/

struct Solution;
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let ns = profits.len();
        let mut projects = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<(i32, i32)>>();
        projects.sort_unstable_by_key(|x| x.0);

        let mut heap = std::collections::BinaryHeap::<i32>::new();

        let mut i = 0;
        for _ in 0..k {
            while i < ns && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }
            if heap.len() == 0 {
                break;
            }
            w += heap.pop().unwrap();
        }

        w
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
    );
    println!(
        "{}",
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2])
    );
}
