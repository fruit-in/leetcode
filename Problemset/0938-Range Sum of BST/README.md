# 938. Range Sum of BST
Given the <code>root</code> node of a binary search tree, return the sum of values of all nodes with value between <code>L</code> and <code>R</code> (inclusive).

The binary search tree is guaranteed to have unique values.

#### Example 1:
<pre>
<strong>Input:</strong> root = [10,5,15,3,7,null,18], L = 7, R = 15
<strong>Output:</strong> 32
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = [10,5,15,3,7,13,18,1,null,6], L = 6, R = 10
<strong>Output:</strong> 23
</pre>

#### Note:
1. The number of nodes in the tree is at most <code>10000</code>.
2. The final answer is guaranteed to be less than <code>2<sup>31</sup></code>.

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
    def rangeSumBST(self, root: TreeNode, L: int, R: int) -> int:
        rangesum = root.val
        if rangesum < L or rangesum > R:
            rangesum = 0
        if root.left:
            rangesum += self.rangeSumBST(root.left, L, R)
        if root.right:
            rangesum += self.rangeSumBST(root.right, L, R)
        return rangesum
```
