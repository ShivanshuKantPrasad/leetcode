// https://leetcode.com/problems/binary-tree-postorder-traversal/

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

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    if let Some(root) = root.as_ref() {
        result.extend(postorder_traversal(root.borrow().left.clone()));
        result.extend(postorder_traversal(root.borrow().right.clone()));
        result.push(root.borrow().val);
    }

    result
}
