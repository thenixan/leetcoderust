use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::ops::Add;
use std::fmt::{Display, Formatter};
use std::error::Error;
use core::fmt::Debug;

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

    pub fn new_from_str(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let v: Vec<&str> = s[1..s.len() - 1].split(",").collect();
        let head_s = v.get(0);
        return match head_s {
            Some(&"null") => None,
            Some(x) => {
                let head = Rc::new(RefCell::new(TreeNode::new(x.parse::<i32>().unwrap())));
                Self::consume(&v, 1, head.clone());
                Some(head.clone())
            }
            None => None
        };
    }

    fn consume(v: &Vec<&str>, mut position: usize, head: Rc<RefCell<TreeNode>>) -> usize {
        if position > v.len() {
            return position;
        }
        let left_s = v.get(position);
        position += 1;
        let right_s = v.get(position);
        position += 1;

        let left: Option<Rc<RefCell<TreeNode>>> = match left_s {
            Some(&"null") => {
                None
            }
            None => None,
            Some(s) => {
                Some(Rc::new(RefCell::new(TreeNode::new(s.parse().unwrap()))))
            }
        };

        let right: Option<Rc<RefCell<TreeNode>>> = match right_s {
            Some(&"null") => {
                None
            }
            None => None,
            Some(s) => {
                Some(Rc::new(RefCell::new(TreeNode::new(s.parse().unwrap()))))
            }
        };

        match &left {
            Some(x) => position = Self::consume(v, position, x.clone()),
            _ => {}
        }
        match &right {
            Some(x) => position = Self::consume(v, position, x.clone()),
            _ => {}
        }


        head.borrow_mut().left = left;
        head.borrow_mut().right = right;
        return position;
    }
}

pub struct Solution {}

#[derive(Debug)]
struct Decision {
    if_rob: i32,
    if_not: i32,
}

impl Decision {
    pub fn new(if_rob: i32, if_not: i32) -> Self { Decision { if_rob, if_not } }
    pub fn empty() -> Self { Decision::new(0, 0) }
}

enum Status {
    Robbed,
    CanRob(i32),
    CannotRob,
    Start,
}

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let decision = Self::iterate(root, 0);
        return i32::max(decision.if_rob, decision.if_not);
    }

    fn iterate(root: Option<Rc<RefCell<TreeNode>>>, parent: i32) -> Decision {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                let l_decision = Self::iterate(node.borrow().left.clone(), val);
                let r_decision = Self::iterate(node.borrow().right.clone(), val);

                let non_rob_profit = l_decision.if_not + r_decision.if_not;

                return Decision::new(val + non_rob_profit, i32::max(l_decision.if_rob, l_decision.if_not) + i32::max(r_decision.if_rob, r_decision.if_not));
            }
            None => { return Decision::empty(); }
        }
    }
}