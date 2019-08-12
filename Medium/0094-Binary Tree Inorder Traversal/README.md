# 94. Binary Tree Inorder Traversal
Given a binary tree, return the *inorder* traversal of its nodes' values.

#### Example:
<pre>
<strong>Input:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>Output:</strong> [1,3,2]
</pre>

<strong>Follow up:</strong> Recursive solution is trivial, could you do it iteratively?

## Solutions (Python)

### 1. Recursion
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return self.inorderTraversal(root.left) + \
            [root.val] + self.inorderTraversal(root.right)
```

### 2. Iteration
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        vals = []
        nodes = []
        node = root
        while node or nodes:
            while node:
                nodes.append(node)
                node = node.left
            node = nodes.pop()
            vals.append(node.val)
            node = node.right
        return vals
```

## Solutions (Rust)

### 1. Recursion
```Rust
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
        if let Some(n) = root {
            vals.append(&mut Self::inorder_traversal(n.borrow().left.clone()));
            vals.push(n.borrow().val);
            vals.append(&mut Self::inorder_traversal(n.borrow().right.clone()));
        }
        vals
    }
}
```

### 2. Iteration
```Rust
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
```
