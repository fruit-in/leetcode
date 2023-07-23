# 687. 最长同值路径
给定一个二叉树，找到最长的路径，这个路径中的每个节点具有相同值。 这条路径可以经过也可以不经过根节点。

**注意:** 两个节点之间的路径长度由它们之间的边数表示。

#### 示例 1:
输入:
```
              5
             / \
            4   5
           / \   \
          1   1   5
```
输出:
```
2
```

#### 示例 2:
输入:
```
              1
             / \
            4   5
           / \   \
          4   4   5
```
输出:
```
2
```

**注意:** 给定的二叉树不超过10000个结点。 树的高度不超过1000。

## 题解 (Python)

### 1. 深度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def longestUnivaluePath(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            longer, longest = 1, 0
            l = helper(root.left)
            r = helper(root.right)

            if root.left and root.val == root.left.val:
                longer = l[0] + 1
                longest += l[0]
            if root.right and root.val == root.right.val:
                longer = max(longer, r[0] + 1)
                longest += r[0]

            return (longer, max(l[1], r[1], longest))

        return helper(root)[1]
```
