# 563. Binary Tree Tilt
Given a binary tree, return the tilt of the **whole tree**.

The tilt of a **tree node** is defined as the **absolute difference** between the sum of all left subtree node values and the sum of all right subtree node values. Null node has tilt 0.

The tilt of the **whole tree** is defined as the sum of all nodes' tilt.

#### Example:
<pre>
<strong>Input:</strong>
         1
       /   \
      2     3
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Tilt of node 2 : 0
Tilt of node 3 : 0
Tilt of node 1 : |2-3| = 1
Tilt of binary tree : 0 + 0 + 1 = 1
</pre>

#### Note:
1. The sum of node values in any subtree won't exceed the range of 32-bit integer. 
2. All the tilt values won't exceed the range of 32-bit integer.

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
    def findTilt(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            l_sum, l_tilt = helper(root.left)
            r_sum, r_tilt = helper(root.right)

            return (root.val + l_sum + r_sum, l_tilt + r_tilt + abs(l_sum - r_sum))

        return helper(root)[1]
```
