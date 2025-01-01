// https://leetcode.com/problems/k-th-smallest-prime-fraction/description/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}
impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pair<A, B>
where
    A: Ord + Eq + PartialEq,
    B: Eq + PartialEq,
{
    cost: A,
    value: B,
}

impl<A, B> PartialOrd for Pair<A, B>
where
    A: Ord + Eq + PartialEq,
    B: Eq + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<A, B> Ord for Pair<A, B>
where
    A: Ord + Eq + PartialEq,
    B: Eq + PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
struct Solution;
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut pq = BinaryHeap::new();
        let mut k = k;

        for i in 0..arr.len() {
            pq.push(Pair {
                cost: OrderedFloat(-1.0 * arr[i] as f64 / *arr.last().unwrap() as f64),
                value: (i, arr.len() - 1),
            });
        }

        while k > 1 {
            let mut cur = pq.pop().unwrap();
            cur.value.1 -= 1;

            pq.push(Pair {
                cost: OrderedFloat(-1.0 * arr[cur.value.0] as f64 / arr[cur.value.1] as f64),
                value: (cur.value.0, cur.value.1),
            });
            k -= 1;
        }
        let res = pq.pop().unwrap();

        vec![arr[res.value.0], arr[res.value.1]]
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3)
    );
    println!("{:?}", Solution::kth_smallest_prime_fraction(vec![1, 7], 1));
}
