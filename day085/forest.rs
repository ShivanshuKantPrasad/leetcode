//https://leetcode.com/problems/delete-nodes-and-return-forest/
// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    pub fn del(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &Vec<i32>,
        is_root: bool,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        let next_root = to_delete.contains(&root.borrow().val);

        if root.borrow().left.is_some() {
            let left = root.borrow().left.clone().unwrap();
            if to_delete.contains(&left.borrow().val) {
                root.borrow_mut().left = None;
            }
            del(Some(left), to_delete, next_root, result);
        }

        if root.borrow().right.is_some() {
            let right = root.borrow().right.clone().unwrap();
            if to_delete.contains(&right.borrow().val) {
                root.borrow_mut().right = None;
            }
            del(Some(right), to_delete, next_root, result);
        }

        if is_root && !next_root {
            result.push(Some(root));
        }
    }

    let mut result = Vec::new();
    del(root, &to_delete, true, &mut result);
    result
}
