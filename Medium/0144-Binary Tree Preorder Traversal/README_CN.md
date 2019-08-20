# 144. 二叉树的前序遍历
给定一个二叉树，返回它的*前序* 遍历。

#### 示例:
<pre>
<strong>输入:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>输出:</strong> [1,2,3]
</pre>

**进阶:** 递归算法很简单，你可以通过迭代算法完成吗？

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
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return [root.val] + \
            self.preorderTraversal(root.left) + \
            self.preorderTraversal(root.right)
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
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        vals = []
        nodes = [root]
        while nodes:
            node = nodes.pop()
            vals.append(node.val)
            if node.right:
                nodes.append(node.right)
            if node.left:
                nodes.append(node.left)
        return vals
```
