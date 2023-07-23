# 107. Binary Tree Level Order Traversal II
Given a binary tree, return the *bottom-up level order* traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).

For example:

Given binary tree ```[3,9,20,null,null,15,7]```,
```
    3
   / \
  9  20
    /  \
   15   7
```

return its bottom-up level order traversal as:
```
[
  [15,7],
  [9,20],
  [3]
]
```

## Solutions (Python)

### 1. BFS
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
