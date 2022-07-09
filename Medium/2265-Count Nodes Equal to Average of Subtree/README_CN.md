# 2265. 统计值等于子树平均值的节点数
给你一棵二叉树的根节点 `root` ，找出并返回满足要求的节点数，要求节点的值等于其 **子树** 中值的 **平均值** 。

#### 注意:
* `n` 个元素的平均值可以由 `n` 个元素 **求和** 然后再除以 `n` ，并 **向下舍入** 到最近的整数。
* `root` 的 **子树** 由 `root` 和它的所有后代组成。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/15/image-20220315203925-1.png)
<pre>
<strong>输入:</strong> root = [4,8,5,0,1,null,6]
<strong>输出:</strong> 5
<strong>解释:</strong>
对值为 4 的节点：子树的平均值 (4 + 8 + 5 + 0 + 1 + 6) / 6 = 24 / 6 = 4 。
对值为 5 的节点：子树的平均值 (5 + 6) / 2 = 11 / 2 = 5 。
对值为 0 的节点：子树的平均值 0 / 1 = 0 。
对值为 1 的节点：子树的平均值 1 / 1 = 1 。
对值为 6 的节点：子树的平均值 6 / 1 = 6 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/26/image-20220326133920-1.png)
<pre>
<strong>输入:</strong> root = [1]
<strong>输出:</strong> 1
<strong>解释:</strong> 对值为 1 的节点：子树的平均值 1 / 1 = 1。
</pre>

#### 提示:
* 树中节点数目在范围 `[1, 1000]` 内
* `0 <= Node.val <= 1000`

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
    def averageOfSubtree(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[2]

    def dfs(self, root: TreeNode) -> (int, int, int):
        if not root:
            return (0, 0, 0)

        count_left, sum_left, ret_left = self.dfs(root.left)
        count_right, sum_right, ret_right = self.dfs(root.right)
        count_root = count_left + count_right + 1
        sum_root = sum_left + sum_right + root.val
        ret_root = ret_left + ret_right + \
            (1 if root.val == sum_root // count_root else 0)

        return (count_root, sum_root, ret_root)
```
