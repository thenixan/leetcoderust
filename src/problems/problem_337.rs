use std::cell::RefCell;
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, sum) = Self::iterate(root, 0);
        return sum;
    }

    fn iterate(root: Option<Rc<RefCell<TreeNode>>>, parent: i32) -> (bool, i32, i32) {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                let (l_can_rob, left_val, left_sum) = Self::iterate(node.borrow().left.clone(), val);
                let (r_can_rob, right_val, right_sum) = Self::iterate(node.borrow().right.clone(), val);
                if l_can_rob && r_can_rob && val > left_val + right_val {
                    return (false, val, val + left_sum + right_sum)
                } else if l_can_rob && r_can_rob {
                    return (true, )
                }
                if left_robbed || right_robbed {
                    return (false, val, left_sum + right_sum);
                } else if val > left_val + right_val {
                    return (true, val, val + left_sum + right_sum);
                } else {
                    return (false, val, left_sum + right_sum);
                }
            }
            None => { return (true, 0, 0); }
        }
    }
}