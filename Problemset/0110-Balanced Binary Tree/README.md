# 110. Balanced Binary Tree
Given a binary tree, determine if it is height-balanced.

For this problem, a height-balanced binary tree is defined as:
> a binary tree in which the depth of the two subtrees of *every* node never differ by more than 1.

#### Example 1:
Given the following tree ```[3,9,20,null,null,15,7]```:
```
    3
   / \
  9  20
    /  \
   15   7
```
Return true.

#### Example 2:
Given the following tree ```[1,2,2,3,3,null,null,4,4]```:
```
       1
      / \
     2   2
    / \
   3   3
  / \
 4   4
```
Return false.

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
    def isBalanced(self, root: TreeNode) -> bool:
        def depth(root: TreeNode) -> int:
            if not root:
                return 0
            ldepth = depth(root.left)
            rdepth = depth(root.right)
            if ldepth == -1 or rdepth == -1 or abs(ldepth - rdepth) > 1:
                return -1
            return max(ldepth, rdepth) + 1

        return depth(root) != -1
```
