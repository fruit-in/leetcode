# 669. 修剪二叉搜索树
给定一个二叉搜索树，同时给定最小边界```L``` 和最大边界 ```R```。通过修剪二叉搜索树，使得所有节点的值在```[L, R]```中 (R>=L) 。你可能需要改变树的根节点，所以结果应当返回修剪好的二叉搜索树的新的根节点。

#### 示例 1:
<pre>
<strong>输入:</strong>
    1
   / \
  0   2

  L = 1
  R = 2
<strong>输出:</strong>
    1
      \
       2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
    3
   / \
  0   4
   \
    2
   /
  1

  L = 1
  R = 3
<strong>输出:</strong>
      3
     /
   2
  /
 1
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
    def trimBST(self, root: TreeNode, L: int, R: int) -> TreeNode:
        if not root:
            return root
        elif root.val > R:
            return self.trimBST(root.left, L, R)
        elif root.val < L:
            return self.trimBST(root.right, L, R)
        else:
            root.left = self.trimBST(root.left, L, R)
            root.right = self.trimBST(root.right, L, R)
            return root
```
