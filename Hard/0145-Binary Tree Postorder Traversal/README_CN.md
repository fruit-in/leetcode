# 145. 二叉树的后序遍历
给定一个二叉树，返回它的*后序*遍历。

#### 示例:
<pre>
<strong>输入:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>输出:</strong> [3,2,1]
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
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return self.postorderTraversal(root.left) + \
            self.postorderTraversal(root.right) + [root.val]
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
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        nset = set()
        vals = []
        nodes = [None]
        node = root
        
        while nodes and node:
            while node.left and node.left not in nset:
                nodes.append(node)
                node = node.left
            if node.right and node.right not in nset:
                nodes.append(node)
                node = node.right
            else:
                vals.append(node.val)
                nset.add(node)
                node = nodes.pop()
        return vals
```
