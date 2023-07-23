# 515. 在每个树行中找最大值
您需要在二叉树的每一行中找到最大的值。

#### 示例:
<pre>
<strong>输入:</strong>
          1
         / \
        3   2
       / \   \
      5   3   9
<strong>输出:</strong> [1, 3, 9]
</pre>

## 题解 (Python)

### 1. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def largestValues(self, root: TreeNode) -> List[int]:
        if not root:
            return []

        curr_level = [root]
        ret = []

        while curr_level:
            ret.append(max(node.val for node in curr_level))
            next_level = [node.left for node in curr_level if node.left]
            next_level.extend(node.right for node in curr_level if node.right)
            curr_level = next_level

        return ret
```
