# 2236. 判断根结点是否等于子结点之和
给你一个 **二叉树** 的根结点 `root`，该二叉树由恰好 `3` 个结点组成：根结点、左子结点和右子结点。

如果根结点值等于两个子结点值之和，返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/08/graph3drawio.png)
<pre>
<strong>输入:</strong> root = [10,4,6]
<strong>输出:</strong> true
<strong>解释:</strong> 根结点、左子结点和右子结点的值分别是 10 、4 和 6 。
由于 10 等于 4 + 6 ，因此返回 true 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/04/08/graph3drawio-1.png)
<pre>
<strong>输入:</strong> root = [5,3,1]
<strong>输出:</strong> false
<strong>解释:</strong> 根结点、左子结点和右子结点的值分别是 5 、3 和 1 。
由于 5 不等于 3 + 1 ，因此返回 false 。
</pre>

#### 提示:
* 树只包含根结点、左子结点和右子结点
* `-100 <= Node.val <= 100`

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def checkTree(self, root: Optional[TreeNode]) -> bool:
        return root.val == root.left.val + root.right.val
```
