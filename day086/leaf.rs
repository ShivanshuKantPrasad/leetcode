// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/description/

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

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    fn post_order(root: Option<Rc<RefCell<TreeNode>>>, distance: usize) -> Vec<i32> {
        if root.is_none() {
            return vec![0; 12];
        }
        let root = root.unwrap();
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            let mut current = vec![0; 12];
            current[0] = 1;
            return current;
        }

        let left = post_order(root.borrow().left.clone(), distance);
        let right = post_order(root.borrow().right.clone(), distance);

        let mut current = vec![0; 12];

        for i in 0..10 {
            current[i + 1] += left[i] + right[i];
        }

        current[11] += left[11] + right[11];

        for d1 in 0..=distance {
            for d2 in 0..=distance {
                if (2 + d1 + d2 <= distance) {
                    current[11] += left[d1] * right[d2];
                }
            }
        }

        return current;
    }
    post_order(root, distance as usize)[11]
}
