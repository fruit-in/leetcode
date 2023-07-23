# 94. 二叉树的中序遍历
给定一个二叉树，返回它的*中序* 遍历。

#### 示例:
<pre>
<strong>输入:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>输出:</strong> [1,3,2]
</pre>

<strong>进阶:</strong> 递归算法很简单，你可以通过迭代算法完成吗？

## 题解 (Python)

### 1. 递归
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

### 2. 迭代
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

## 题解 (Rust)

### 1. 递归
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

### 2. 迭代
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
