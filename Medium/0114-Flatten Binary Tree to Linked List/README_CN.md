# 114. 二叉树展开为链表
给定一个二叉树，[原地](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95/8010757)将它展开为一个单链表。

例如，给定二叉树
```
    1
   / \
  2   5
 / \   \
3   4   6
```

将其展开为：
```
1
 \
  2
   \
    3
     \
      4
       \
        5
         \
          6
```

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def flatten(self, root: TreeNode) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        if not root:
            return None

        stack = [root]
        prev = TreeNode(left=root)

        while stack:
            curr = stack.pop()

            prev.left = None
            prev.right = curr

            if curr.right:
                stack.append(curr.right)
            if curr.left:
                stack.append(curr.left)

            prev = curr
```
