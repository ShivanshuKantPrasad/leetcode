// https://leetcode.com/problems/delete-leaves-with-a-given-value/

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

impl From<(i32, Option<TreeNode>, Option<TreeNode>)> for TreeNode {
    fn from(value: (i32, Option<TreeNode>, Option<TreeNode>)) -> Self {
        Self {
            val: value.0,
            left: if value.1.is_some() {
                Some(Rc::new(RefCell::new(value.1.unwrap())))
            } else {
                None
            },
            right: if value.2.is_some() {
                Some(Rc::new(RefCell::new(value.2.unwrap())))
            } else {
                None
            },
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref cell) = root {
            let mut node = cell.borrow_mut();
            node.left = Self::remove_leaf_nodes(node.left.clone(), target);
            node.right = Self::remove_leaf_nodes(node.right.clone(), target);
            if node.left.is_none() && node.right.is_none() && node.val == target {
                return None;
            }
        }
        root
    }
}

fn main() {
    let tree = TreeNode::from((
        1,
        Some(TreeNode::from((2, Some(TreeNode::new(2)), None))),
        Some(TreeNode::from((
            3,
            Some(TreeNode::new(2)),
            Some(TreeNode::new(4)),
        ))),
    ));
    println!("{:#?}", tree);
    println!(
        "{:#?}",
        Solution::remove_leaf_nodes(Some(Rc::new(RefCell::new(tree))), 2)
    );
}
