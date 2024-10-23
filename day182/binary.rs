// https://leetcode.com/problems/cousins-in-binary-tree-ii/submissions/1431497399/
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

pub fn replace_value_in_tree(
    mut root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    pub fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(root) = root {
            let left = &root.borrow().left;
            let right = &root.borrow().right;
            usize::max(depth(right), depth(left)) + 1
        } else {
            0
        }
    }
    pub fn sum(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, sums: &mut Vec<i32>) {
        if let Some(root) = root {
            let mut root = root.borrow();
            sum(&root.left, level + 1, sums);
            sum(&root.right, level + 1, sums);
            sums[level] += root.val;
        }
    }

    pub fn update(root: &mut Option<Rc<RefCell<TreeNode>>>, level: usize, sums: &Vec<i32>) {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            update(&mut root.left, level + 1, sums);
            update(&mut root.right, level + 1, sums);

            let sibling_sum = root
                .left
                .as_ref()
                .and_then(|left| Some(left.borrow().val))
                .unwrap_or(0)
                + root
                    .right
                    .as_ref()
                    .and_then(|right| Some(right.borrow().val))
                    .unwrap_or(0);

            if level + 1 < sums.len() {
                let sum = sums[level + 1] - sibling_sum;
                root.left.as_mut().map(|left| left.borrow_mut().val = sum);
                root.right
                    .as_mut()
                    .map(|right| right.borrow_mut().val = sum);
            }
        }
    }

    let depth = depth(root);
    let mut sums = vec![0; depth];
    sum(&root, 0, &mut sums);
    update(&mut root, 0, &sums);
    root.as_mut().map(|root| root.borrow_mut().val = 0);
    root
}
