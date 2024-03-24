// 2331. Evaluate Boolean Binary Tree

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    fn create_node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right,
        }))
    }

    let root = Some(create_node(2,
        Some(create_node(1, None, None)),
        Some(create_node(3,
            Some(create_node(0, None, None)),
            Some(create_node(1, None, None))
        ))
    ));
    
    let result = Solution::evaluate_tree(root);

    println!("result = {}", result);
}

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
            right: None
        }
    }
}

struct Solution;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Leaf nodes have either the value 0 or 1, where 0 represents False and 1 represents True.
        // Non-leaf nodes have either the value 2 or 3, where 2 represents the boolean OR and 3 represents the boolean AND.
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return node.val == 1;
            }
            let left = Solution::evaluate_tree(node.left.clone());
            let right = Solution::evaluate_tree(node.right.clone());
            match node.val {
                2 => left || right,
                3 => left && right,
                _ => false,
            }
        } else {
            false
        }
    }
}