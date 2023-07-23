# 543. Diameter of Binary Tree
Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the **longest** path between any two nodes in a tree. This path may or may not pass through the root.

#### Example:
Given a binary tree
```
          1
         / \
        2   3
       / \
      4   5
```
Return **3**, which is the length of the path [4,2,1,3] or [5,2,1,3].

**Note:** The length of path between two nodes is represented by the number of edges between them.

## Solutions (Python)

### 1. DFS
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (-1, 0)

            l_longer, l_diameter = helper(root.left)
            r_longer, r_diameter = helper(root.right)

            l_longer += 1
            r_longer += 1

            return (max(l_longer, r_longer), max(l_diameter, r_diameter, l_longer + r_longer))


        return helper(root)[1]
```
