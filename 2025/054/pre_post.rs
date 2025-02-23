//https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
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

pub fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() != postorder.len() {
        return None;
    }
    let n = preorder.len();

    let (&left_root, preorder) = preorder.split_first()?;
    let (&right_root, postorder) = postorder.split_last()?;

    if left_root != right_root {
        return None;
    }

    let mut root = TreeNode::new(left_root);

    if n == 1 {
        return Some(Rc::new(RefCell::new(root)));
    }

    if n == 2 {
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        return Some(Rc::new(RefCell::new(root)));
    }

    let left_val = *preorder.first()?;
    let right_val = *postorder.last()?;

    let left_tree_end = preorder.iter().position(|&x| x == right_val)?;
    let (left_pre, right_pre) = preorder.split_at(left_tree_end);

    let right_tree_end = postorder.iter().position(|&x| x == left_val)?;
    let (left_post, right_post) = postorder.split_at(left_tree_end);

    root.left = construct_from_pre_post(left_pre.into(), left_post.into());
    root.right = construct_from_pre_post(right_pre.into(), right_post.into());

    Some(Rc::new(RefCell::new(root)))
}
