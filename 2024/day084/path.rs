// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/description/
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

pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    pub fn get_direction(root: Option<Rc<RefCell<TreeNode>>>, value: i32) -> String {
        if root == None {
            return String::new();
        }
        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        match left {
            Some(left) => {
                if value == left.borrow().val {
                    return String::from("L");
                }
                let left = get_direction(Some(left.clone()), value);
                if !left.is_empty() {
                    return String::from("L") + &left;
                }
            }
            None => {}
        }

        match right {
            Some(right) => {
                if value == right.borrow().val {
                    return String::from("R");
                }
                let right = get_direction(Some(right.clone()), value);
                if !right.is_empty() {
                    return String::from("R") + &right;
                }
            }
            None => {}
        }

        String::new()
    }

    let start = get_direction(root.clone(), start_value);
    let start = start.as_bytes();
    let dest = get_direction(root, dest_value);
    let dest = dest.as_bytes();

    let sl = start.len();
    let dl = dest.len();

    let mut i = 0;

    while i < sl && i < dl && start[i] == dest[i] {
        i += 1;
    }

    let mut result = Vec::new();
    for j in i..sl {
        result.push(b'U');
    }

    for j in i..dl {
        result.push(dest[j]);
    }

    String::from_utf8(result).unwrap()
}
