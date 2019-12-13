# 637. 二叉树的层平均值
给定一个非空二叉树, 返回一个由每层节点平均值组成的数组.

#### 示例 1:
<pre>
<strong>输入:</strong>
    3
   / \
  9  20
    /  \
   15   7
<strong>输出:</strong> [3, 14.5, 11]
<strong>解释:</strong>
第0层的平均值是 3,  第1层是 14.5, 第2层是 11. 因此返回 [3, 14.5, 11].
</pre>

#### 注意:
1. 节点值的范围在32位有符号整数范围内。

## 题解 (Python)

### 1. 广度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        averages = []
        curr_level = [root]

        while curr_level:
            averages.append(sum(node.val for node in curr_level) / len(curr_level))

            temp = [node.left for node in curr_level if node.left]
            temp.extend(node.right for node in curr_level if node.right)
            curr_level = temp

        return averages
```
