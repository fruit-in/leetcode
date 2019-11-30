# 530. 二叉搜索树的最小绝对差
给定一个所有节点为非负值的二叉搜索树，求树中任意两节点的差的绝对值的最小值。

#### 示例:
<pre>
<strong>输入:</strong>
   1
    \
     3
    /
   2
<strong>输出:</strong>
1
<strong>解释:</strong>
最小绝对差为1，其中 2 和 1 的差的绝对值为 1（或者 2 和 3）。
</pre>

**注意:** 树中至少有2个节点。

## 题解 (Python)

### 1. 中序遍历
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getMinimumDifference(self, root: TreeNode) -> int:
        nodes = []
        curr = root
        prev = float("-inf")
        min_diff = float("+inf")

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            min_diff = min(min_diff, curr.val - prev)
            prev = curr.val

            curr = curr.right

        return min_diff
```
