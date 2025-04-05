// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/description/
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

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    pub fn post(
        root: Option<Rc<RefCell<TreeNode>>>,
        deepest: &mut i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        *deepest += 1;

        let mut left_depth = *deepest;
        let left_deepest = post(left, &mut left_depth);

        let mut right_depth = *deepest;
        let right_deepest = post(right, &mut right_depth);

        *deepest = left_depth.max(right_depth);
        if left_depth == right_depth {
            return Some(root);
        } else if left_depth > right_depth {
            return left_deepest;
        } else {
            return right_deepest;
        }
    }

    let mut depth = 0;
    post(root, &mut 0)
}
