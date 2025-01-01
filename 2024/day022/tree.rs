// https://leetcode.com/problems/evaluate-boolean-binary-tree/description/

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

struct Solution;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            if node.as_ref().borrow_mut().val == 0 {
                return false;
            }
            if node.as_ref().borrow_mut().val == 1 {
                return true;
            }
            if node.as_ref().borrow_mut().val == 2 {
                return Self::evaluate_tree(node.as_ref().borrow_mut().right.clone())
                    || Self::evaluate_tree(node.as_ref().borrow_mut().left.clone());
            };

            if node.as_ref().borrow_mut().val == 3 {
                return Self::evaluate_tree(node.as_ref().borrow_mut().right.clone())
                    && Self::evaluate_tree(node.as_ref().borrow_mut().left.clone());
            }
            false
        } else {
            false
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::evaluate_tree(Some(Rc::new(RefCell::new(TreeNode::new(0)))))
    );
    println!(
        "{}",
        Solution::evaluate_tree(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })))
        }))))
    );
}
