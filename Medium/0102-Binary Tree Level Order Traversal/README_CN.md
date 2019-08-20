# 102. 二叉树的层次遍历
给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。

例如:
给定二叉树: <code>[3,9,20,null,null,15,7]</code>,
<pre>
    3
   / \
  9  20
    /  \
   15   7
</pre>
返回其层次遍历结果:
<pre>
[
  [3],
  [9,20],
  [15,7]
]
</pre>

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
    def levelOrder(self, root: TreeNode) -> List[List[int]]:
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
        return ret
```
