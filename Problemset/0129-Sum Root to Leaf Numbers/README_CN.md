# 129. 求根到叶子节点数字之和
给定一个二叉树，它的每个结点都存放一个 ```0-9``` 的数字，每条从根到叶子节点的路径都代表一个数字。

例如，从根到叶子节点路径 ```1->2->3``` 代表数字 ```123```。

计算从根到叶子节点生成的所有数字之和。

**说明:** 叶子节点是指没有子节点的节点。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3]
    1
   / \
  2   3
<strong>输出:</strong> 25
<strong>解释:</strong>
从根到叶子节点路径 1->2 代表数字 12.
从根到叶子节点路径 1->3 代表数字 13.
因此，数字总和 = 12 + 13 = 25.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [4,9,0,5,1]
    4
   / \
  9   0
 / \
5   1
<strong>输出:</strong> 1026
<strong>解释:</strong>
从根到叶子节点路径 4->9->5 代表数字 495.
从根到叶子节点路径 4->9->1 代表数字 491.
从根到叶子节点路径 4->0 代表数字 40.
因此，数字总和 = 495 + 491 + 40 = 1026.
</pre>

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumNumbers(self, root: TreeNode) -> int:
        def helper(root: TreeNode, n: int) -> int:
            n = 10 * n + root.val

            if root.left and root.right:
                return helper(root.left, n) + helper(root.right, n)
            elif root.left:
                return helper(root.left, n)
            elif root.right:
                return helper(root.right, n)
            else:
                return n

        return helper(root, 0) if root else 0
```
