# 938. 二叉搜索树的范围和
给定二叉搜索树的根结点 ```root```，返回 ```L``` 和 ```R```（含）之间的所有结点的值的和。

二叉搜索树保证具有唯一的值。

#### 示例 1:
<pre>
<strong>输入:</strong> root = [10,5,15,3,7,null,18], L = 7, R = 15
<strong>输出:</strong> 32
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = [10,5,15,3,7,13,18,1,null,6], L = 6, R = 10
<strong>输出:</strong> 23
</pre>

#### 提示:
1. 树中的结点数量最多为 ```10000``` 个。
2. 最终的答案保证小于 ```2^31```。

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
    def rangeSumBST(self, root: TreeNode, L: int, R: int) -> int:
        rangesum = root.val
        if rangesum < L or rangesum > R:
            rangesum = 0
        if root.left:
            rangesum += self.rangeSumBST(root.left, L, R)
        if root.right:
            rangesum += self.rangeSumBST(root.right, L, R)
        return rangesum
```
