# 530. Minimum Absolute Difference in BST
Given a binary search tree with non-negative values, find the minimum [absolute difference](https://en.wikipedia.org/wiki/Absolute_difference) between values of any two nodes.

#### Example:
<pre>
<strong>Input:</strong>
   1
    \
     3
    /
   2
<strong>Output:</strong>
1
<strong>Explanation:</strong>
The minimum absolute difference is 1, which is the difference between 2 and 1 (or between 2 and 3).
</pre>

**Note:** There are at least two nodes in this BST.

## Solutions (Python)

### 1. Inorder Traversal
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getMinimumDifference(self, root: TreeNode) -> int:
        nodes = []
        curr = root
        prev = float("-inf")
        min_diff = float("+inf")

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            min_diff = min(min_diff, curr.val - prev)
            prev = curr.val

            curr = curr.right

        return min_diff
```
