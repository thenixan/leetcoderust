use core::borrow::Borrow;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;
use std::str::FromStr;

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