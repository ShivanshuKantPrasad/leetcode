// https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/
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

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    let mut res = Vec::<i32>::new();

    let mut last_row = 0;

    if let Some(root) = root {
        queue.push_back((root.clone(), 0));
        res.push(i32::MIN);
        while let Some((root, cur_row)) = queue.pop_front() {
            let root = root.borrow();
            if cur_row == last_row {
                res[cur_row] = res[cur_row].max(root.val);
            } else {
                res.push(root.val);
                last_row = cur_row;
            }
            if let Some(left) = &root.left {
                queue.push_back((left.clone(), cur_row + 1))
            }

            if let Some(right) = &root.right {
                queue.push_back((right.clone(), cur_row + 1))
            }
        }
        res
    } else {
        vec![]
    }
}
