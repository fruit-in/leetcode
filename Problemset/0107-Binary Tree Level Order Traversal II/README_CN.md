# 107. 二叉树的层次遍历 II
给定一个二叉树，返回其节点值自底向上的层次遍历。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）

例如：

给定二叉树 ```[3,9,20,null,null,15,7]```,
```
    3
   / \
  9  20
    /  \
   15   7
```

返回其自底向上的层次遍历为：
```
[
  [15,7],
  [9,20],
  [3]
]
```

## 题解 (Python)

### 1. 广度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def levelOrderBottom(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []
        ret = []
        level = [root]
        while level:
            vals = []
            children = []
            for node in level:
                vals.append(node.val)
                if node.left:
                    children.append(node.left)
                if node.right:
                    children.append(node.right)
            ret.append(vals)
            level = children
        return ret[::-1]
```
