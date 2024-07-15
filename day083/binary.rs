// https://leetcode.com/problems/create-binary-tree-from-descriptions/

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

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::{HashMap, HashSet};
    let mut hash = HashMap::<i32, Rc<RefCell<TreeNode>>>::new();
    let mut nodes = HashSet::<i32>::new();
    let mut children = HashSet::<i32>::new();

    for description in descriptions {
        let parent = description[0];
        let child = description[1];

        nodes.insert(parent);
        nodes.insert(child);
        children.insert(child);

        let parent = hash
            .entry(parent)
            .or_insert(Rc::new(RefCell::new(TreeNode::new(parent))))
            .clone();
        let child = hash
            .entry(child)
            .or_insert(Rc::new(RefCell::new(TreeNode::new(child))))
            .clone();

        let is_left = description[2];

        if is_left == 1 {
            parent.borrow_mut().left = Some(child);
        } else {
            parent.borrow_mut().right = Some(child);
        }
    }

    let parent = nodes.difference(&children).next().unwrap();
    hash.get(&parent).cloned()
}
