// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/description/

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    pub fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(root) = root {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            usize::max(depth(right), depth(left)) + 1
        } else {
            0
        }
    }
    pub fn sum(root: Option<Rc<RefCell<TreeNode>>>, level: usize, sums: &mut Vec<i64>) {
        if let Some(root) = root {
            let root = root.borrow();
            sums[level] += root.val as i64;
            sum(root.left.clone(), level + 1, sums);
            sum(root.right.clone(), level + 1, sums);
        }
    }

    let depth = depth(root.clone());
    if k > depth as i32 {
        return -1;
    }
    let mut sums = vec![0; depth];
    sum(root, 0, &mut sums);
    sums.sort_unstable_by_key(|x| -x);
    sums[k as usize - 1]
}
