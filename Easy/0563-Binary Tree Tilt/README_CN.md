# 563. 二叉树的坡度
给定一个二叉树，计算**整个树**的坡度。

一个树的**节点的坡度**定义即为，该节点左子树的结点之和和右子树结点之和的**差的绝对值**。空结点的的坡度是0。

**整个树**的坡度就是其所有节点的坡度之和。

#### 示例:
<pre>
<strong>输入:</strong>
         1
       /   \
      2     3
<strong>输出:</strong> 1
<strong>解释:</strong>
结点的坡度 2 : 0
结点的坡度 3 : 0
结点的坡度 1 : |2-3| = 1
树的坡度 : 0 + 0 + 1 = 1
</pre>

#### 注意:
1. 任何子树的结点的和不会超过32位整数的范围。
2. 坡度的值不会超过32位整数的范围。

## 题解 (Python)

### 1. 递归
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTilt(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            l_sum, l_tilt = helper(root.left)
            r_sum, r_tilt = helper(root.right)

            return (root.val + l_sum + r_sum, l_tilt + r_tilt + abs(l_sum - r_sum))

        return helper(root)[1]
```
