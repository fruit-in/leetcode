# 111. 二叉树的最小深度
给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

**说明:** 叶子节点是指没有子节点的节点。

#### 示例:
给定二叉树 ```[3,9,20,null,null,15,7]```,
<pre>
    3
   / \
  9  20
    /  \
   15   7
</pre>
返回它的最小深度  2.

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
    def minDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        if not root.left and not root.right:
            return 1
        lmin, rmin = 0, 0
        if root.left:
            lmin = self.minDepth(root.left) + 1
        if root.right:
            rmin = self.minDepth(root.right) + 1
        if lmin and rmin:
            return min(lmin, rmin)
        else:
            return lmin if lmin else rmin
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
    def minDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        depth = 1
        nodes = [root]
        while True:
            for node in nodes:
                if not node.left and not node.right:
                    return depth
            depth += 1
            nodes = [node.left for node in nodes if node.left] \
                + [node.right for node in nodes if node.right]
```
