# 257. 二叉树的所有路径
给定一个二叉树，返回所有从根节点到叶子节点的路径。

**说明:** 叶子节点是指没有子节点的节点。

#### 示例:
<pre>
<strong>输入:</strong>

   1
 /   \
2     3
 \
  5

<strong>输出:</strong> ["1->2->5", "1->3"]
<strong>解释:</strong> 所有根节点到叶子节点的路径为: 1->2->5, 1->3
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
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []
        if not root.left and not root.right:
            return [str(root.val)]

        paths = self.binaryTreePaths(root.left)
        paths.extend(self.binaryTreePaths(root.right))

        return ["%d->" % root.val + path for path in paths]
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
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []

        paths = []
        nodes = [(root, str(root.val))]

        while nodes:
            curr, path = nodes.pop()

            if not curr.left and not curr.right:
                paths.append(path)
            if curr.left:
                nodes.append((curr.left, path + "->" + str(curr.left.val)))
            if curr.right:
                nodes.append((curr.right, path + "->" + str(curr.right.val)))

        return paths
```
