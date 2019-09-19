# 104. 二叉树的最大深度
给定一个二叉树，找出其最大深度。

二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。

**说明:** 叶子节点是指没有子节点的节点。

#### 示例:
给定二叉树 ```[3,9,20,null,null,15,7]```，
<pre>
    3
   / \
  9  20
    /  \
   15   7
</pre>
返回它的最大深度 3 。

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
    def maxDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        lmax = self.maxDepth(root.left)
        rmax = self.maxDepth(root.right)
        return max(lmax, rmax) + 1
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
    def maxDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        depth = 0
        nodes = [root]
        while nodes:
            depth += 1
            nodes = [node.left for node in nodes if node.left] \
                + [node.right for node in nodes if node.right]
        return depth
```
