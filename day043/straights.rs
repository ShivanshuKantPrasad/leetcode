// https://leetcode.com/problems/hand-of-straights/

use std::collections::BTreeMap;

struct Solution;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        let mut freq = BTreeMap::<i32, i32>::new();

        for num in hand {
            *freq.entry(num).or_default() += 1;
        }

        while let Some(mut entry) = freq.first_entry() {
            *entry.get_mut() -= 1;
            let key = *entry.key();
            if *entry.get() == 0 {
                entry.remove();
            }

            for i in 1..group_size {
                if let Some(v) = freq.get_mut(&(key + i)) {
                    *v -= 1;
                    if *v == 0 {
                        freq.remove(&(key + i));
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
    );
    println!("{}", Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
}
