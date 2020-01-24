# 98. Validate Binary Search Tree
Given a binary tree, determine if it is a valid binary search tree (BST).

Assume a BST is defined as follows:
* The left subtree of a node contains only nodes with keys **less than** the node's key.
* The right subtree of a node contains only nodes with keys **greater than** the node's key.
* Both the left and right subtrees must also be binary search trees.

#### Example 1:
<pre>
    2
   / \
  1   3
<strong>Input:</strong> [2,1,3]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
    5
   / \
  1   4
     / \
    3   6
<strong>Input:</strong> [5,1,4,null,null,3,6]
<strong>Output:</strong> false
<strong>Explanation:</strong> The root node's value is 5 but its right child's value is 4.
</pre>

## Solutions (Python)

### 1. Inorder Traversal
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isValidBST(self, root: TreeNode) -> bool:
        nodes = []
        curr = root
        prev_val = float('-inf')

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            if prev_val >= curr.val:
                return False
            prev_val = curr.val

            curr = curr.right

        return True
```
