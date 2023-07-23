# 404. Sum of Left Leaves
Find the sum of all left leaves in a given binary tree.

#### Example:
<pre>
    3
   / \
  9  20
    /  \
   15   7

There are two left leaves in the binary tree, with values <strong>9</strong> and <strong>15</strong> respectively. Return <strong>24</strong>.
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
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        if not root:
            return 0

        s = 0
        if root.left and not root.left.left and not root.left.right:
            s += root.left.val
        elif root.left:
            s += self.sumOfLeftLeaves(root.left)
        if root.right:
            s += self.sumOfLeftLeaves(root.right)

        return s
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
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        if not root:
            return 0

        nodes = [(root, False)]
        s = 0

        while nodes:
            cur, left = nodes.pop()
            if cur.left:
                nodes.append((cur.left, True))
            if cur.right:
                nodes.append((cur.right, False))
            if left and not cur.left and not cur.right:
                s += cur.val

        return s
```
