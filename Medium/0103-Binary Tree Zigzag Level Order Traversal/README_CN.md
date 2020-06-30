# 103. 二叉树的锯齿形层次遍历
给定一个二叉树，返回其节点值的锯齿形层次遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。

例如：
给定二叉树 `[3,9,20,null,null,15,7]`,
```
    3
   / \
  9  20
    /  \
   15   7
```

返回锯齿形层次遍历如下：
```
[
  [3],
  [20,9],
  [15,7]
]
```

## 题解 (Python)

### 1. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def zigzagLevelOrder(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []

        curr_level = [root]
        direction = 1
        ret = []

        while curr_level:
            curr_vals = []
            next_level = []

            for node in curr_level:
                curr_vals.append(node.val)
                if node.left:
                    next_level.append(node.left)
                if node.right:
                    next_level.append(node.right)

            ret.append(curr_vals[::direction])
            direction = -direction
            curr_level = next_level

        return ret
```
