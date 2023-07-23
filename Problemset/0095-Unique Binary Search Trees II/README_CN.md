# 95. 不同的二叉搜索树 II
给定一个整数 *n*，生成所有由 1 ... *n* 为节点所组成的 **二叉搜索树** 。

#### 示例:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong>
[
  [1,null,3,2],
  [3,2,null,1],
  [3,1,null,null,2],
  [2,1,3],
  [1,null,2,null,3]
]
<strong>解释:</strong>
以上的输出对应以下 5 种不同结构的二叉搜索树：

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
</pre>

#### 提示:
* `0 <= n <= 8`

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def generateTrees(self, n: int) -> List[TreeNode]:
        def helper(m: int, n: int) -> List[TreeNode]:
            if m > n:
                return [None]

            ret = []

            for val in range(m, n + 1):
                for left in helper(m, val - 1):
                    for right in helper(val + 1, n):
                        ret.append(TreeNode(val, left, right))

            return ret

        return helper(1, n) if n > 0 else []
```
