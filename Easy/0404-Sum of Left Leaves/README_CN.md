# 404. 左叶子之和
计算给定二叉树的所有左叶子之和。

#### 示例:
<pre>
    3
   / \
  9  20
    /  \
   15   7

在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24
</pre>

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

### 2. 迭代
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
