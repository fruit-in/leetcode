# 543. 二叉树的直径
给定一棵二叉树，你需要计算它的直径长度。一棵二叉树的直径长度是任意两个结点路径长度中的最大值。这条路径可能穿过根结点。

#### 示例:
给定二叉树
```
          1
         / \
        2   3
       / \
      4   5
```
返回 **3**, 它的长度是路径 [4,2,1,3] 或者 [5,2,1,3]。

**注意:** 两结点之间的路径长度是以它们之间边的数目表示。

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
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (-1, 0)

            l_longer, l_diameter = helper(root.left)
            r_longer, r_diameter = helper(root.right)

            l_longer += 1
            r_longer += 1

            return (max(l_longer, r_longer), max(l_diameter, r_diameter, l_longer + r_longer))


        return helper(root)[1]
```
