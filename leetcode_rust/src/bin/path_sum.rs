use std::rc::Rc;
use std::cell::RefCell;

// 112. Path Sum

fn main () {
    // Create a binary tree
    let root = Some(create_node(5,
        Some(create_node(4,
            Some(create_node(11,
                Some(create_node(7, None, None)),
                Some(create_node(2, None, None))
            )),
            None
        )),
        Some(create_node(8,
            Some(create_node(13,
                Some(create_node(5, None, None)),
                Some(create_node(1, None, None))
            )),
            Some(create_node(4, None,
                Some(create_node(1, None, None))
            ))
        ))
    ));

    let target_sum = 22;

    // Check if there's a path sum equals to target_sum
    let result = Solution::has_path_sum(root, target_sum);

    println!("result = {}", result);
}

// Function to create a new tree node
fn create_node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        val,
        left,
        right,
    }))
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
 
impl TreeNode {
  // Function to create a new TreeNode
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
    // Function to check if there's a path sum equals to target_sum
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return node.val == target_sum;
            }
            let left = Solution::has_path_sum(node.left.clone(), target_sum - node.val);
            let right = Solution::has_path_sum(node.right.clone(), target_sum - node.val);
            left || right
        } else {
            false
        }
    }
}