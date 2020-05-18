# 1161. 最大层内元素和
给你一个二叉树的根节点 `root`。设根节点位于二叉树的第 `1` 层，而根节点的子节点位于第 `2` 层，依此类推。

请你找出层内元素之和 **最大** 的那几层（可能只有一层）的层号，并返回其中 **最小** 的那个。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/08/17/capture.jpeg)
<pre>
<strong>输入:</strong> [1,7,0,7,-8,null,null]
<strong>输出:</strong> 2
<strong>解释:</strong>
第 1 层各元素之和为 1，
第 2 层各元素之和为 7 + 0 = 7，
第 3 层各元素之和为 7 + -8 = -1，
所以我们返回第 2 层的层号，它的层内元素之和最大。
</pre>

#### 提示:
1. 树中的节点数介于 `1` 和 `10^4` 之间
2. `-10^5 <= node.val <= 10^5`

## 题解 (Python)

### 1. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxLevelSum(self, root: TreeNode) -> int:
        curr_level = [root]
        max_sum = root.val
        x = 1
        ret = 1

        while curr_level:
            curr_sum = sum(n.val for n in curr_level)
            if curr_sum > max_sum:
                max_sum = curr_sum
                ret = x

            curr_level = [c for n in curr_level for c in [n.left, n.right] if c]
            x += 1

        return ret
```
