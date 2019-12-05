# 669. Trim a Binary Search Tree
Given a binary search tree and the lowest and highest boundaries as ```L``` and ```R```, trim the tree so that all its elements lies in ```[L, R]``` (R >= L). You might need to change the root of the tree, so the result should return the new root of the trimmed binary search tree.

#### Example 1:
<pre>
<strong>Input:</strong>
    1
   / \
  0   2

  L = 1
  R = 2
<strong>Output:</strong>
    1
      \
       2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
    3
   / \
  0   4
   \
    2
   /
  1

  L = 1
  R = 3
<strong>Output:</strong>
      3
     /
   2
  /
 1
</pre>

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
    def trimBST(self, root: TreeNode, L: int, R: int) -> TreeNode:
        if not root:
            return root
        elif root.val > R:
            return self.trimBST(root.left, L, R)
        elif root.val < L:
            return self.trimBST(root.right, L, R)
        else:
            root.left = self.trimBST(root.left, L, R)
            root.right = self.trimBST(root.right, L, R)
            return root
```
