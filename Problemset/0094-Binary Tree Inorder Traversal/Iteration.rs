// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = Vec::new();
        let mut nodes = Vec::new();
        let mut cur = root.clone();
        while !nodes.is_empty() || cur.is_some() {
            while let Some(n) = cur {
                nodes.push(n.clone());
                cur = n.borrow().left.clone();
            }
            cur = nodes.pop();
            if let Some(n) = cur {
                vals.push(n.borrow().val);
                cur = n.borrow().right.clone();
            }
        }
        vals
    }
}
