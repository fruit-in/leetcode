# 1339. 分裂二叉树的最大乘积
给你一棵二叉树，它的根为 `root` 。请你删除 1 条边，使二叉树分裂成两棵子树，且它们子树和的乘积尽可能大。

由于答案可能会很大，请你将结果对 10^9 + 7 取模后再返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/21/sample_1_1699.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6]
<strong>输出:</strong> 110
<strong>解释:</strong> 删除红色的边，得到 2 棵子树，和分别为 11 和 10 。它们的乘积是 110 （11*10）
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/01/21/sample_2_1699.png)
<pre>
<strong>输入:</strong> root = [1,null,2,3,4,null,null,5,6]
<strong>输出:</strong> 90
<strong>解释:</strong> 移除红色的边，得到 2 棵子树，和分别是 15 和 6 。它们的乘积为 90 （15*6）
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [2,3,9,10,7,8,6,5,4,11,1]
<strong>输出:</strong> 1025
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> root = [1,1]
<strong>输出:</strong> 1
</pre>

#### 提示:
* 每棵树最多有 `50000` 个节点，且至少有 `2` 个节点。
* 每个节点的值在 `[1, 10000]` 之间。

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
    def maxProduct(self, root: Optional[TreeNode]) -> int:
        def subtreeSum(root: Optional[TreeNode]) -> int:
            if root is None:
                return 0

            root.val += subtreeSum(root.left)
            root.val += subtreeSum(root.right)

            return root.val

        subtreeSum(root)

        nodes = [root]
        ret = 0

        while nodes != []:
            curr = nodes.pop()
            ret = max(ret, curr.val * (root.val - curr.val))

            if curr.left is not None:
                nodes.append(curr.left)
            if curr.right is not None:
                nodes.append(curr.right)

        return ret % 1000000007
```
