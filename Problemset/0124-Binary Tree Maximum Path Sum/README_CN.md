# 124. 二叉树中的最大路径和
二叉树中的 **路径** 被定义为一条节点序列，序列中每对相邻节点之间都存在一条边。同一个节点在一条路径序列中 **至多出现一次** 。该路径 **至少包含一个** 节点，且不一定经过根节点。

**路径和** 是路径中各节点值的总和。

给你一个二叉树的根节点 `root` ，返回其 **最大路径和** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg)
<pre>
<strong>输入:</strong> root = [1,2,3]
<strong>输出:</strong> 6
<strong>解释:</strong> 最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg)
<pre>
<strong>输入:</strong> root = [-10,9,20,null,null,15,7]
<strong>输出:</strong> 42
<strong>解释:</strong> 最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
</pre>

#### 提示:
* 树中节点数目范围是 <code>[1, 3 * 10<sup>4</sup>]</code>.
* `-1000 <= Node.val <= 1000`

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
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def dfs(root: Optional[TreeNode]) -> (int, int):
            if root is None:
                return (-1000, 0)

            lsubmax, lchildmax = dfs(root.left)
            rsubmax, rchildmax = dfs(root.right)
            submax = max(lsubmax, rsubmax, lchildmax + rchildmax + root.val)
            rootmax = max(0, root.val + max(lchildmax, rchildmax))

            return (submax, rootmax)

        return dfs(root)[0]
```
