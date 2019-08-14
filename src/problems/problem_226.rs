use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {}

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return root.map(|node| {
            let left = Self::invert_tree(node.borrow_mut().left.clone());
            let right = Self::invert_tree(node.borrow_mut().right.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            node
        });
    }
}