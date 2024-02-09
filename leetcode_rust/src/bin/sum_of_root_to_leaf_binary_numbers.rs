// 1022. Sum of Root To Leaf Binary Numbers

use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(0);
    let mut right = TreeNode::new(1);
    let left_left = TreeNode::new(0);
    let left_right = TreeNode::new(1);
    let right_left = TreeNode::new(0);
    let right_right = TreeNode::new(1);
    left.left = Some(Rc::new(RefCell::new(left_left)));
    left.right = Some(Rc::new(RefCell::new(left_right)));
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{}", Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(root)))));
}

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
            right: None
        }
    }
}

struct Solution;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let int = 0;
        let mut sum = 0;
        if let Some(n) = root {
            Self::dfs(n, int, &mut sum);
        }
        sum
    }

    fn dfs(n: Rc<RefCell<TreeNode>>, int: i32, sum: &mut i32) {
        let next_int = (int << 1) | n.borrow().val;
        if n.borrow().left.is_none() && n.borrow().right.is_none() {
            *sum += next_int;
        } else {
            if let Some(l) = n.borrow().left.clone() {
                Self::dfs(l, next_int, sum);
            }
            if let Some(r) = n.borrow().right.clone() {
                Self::dfs(r, next_int, sum);           
            }          
        }

    }
}