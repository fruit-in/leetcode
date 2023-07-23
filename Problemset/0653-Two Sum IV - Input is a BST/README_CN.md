# 653. 两数之和 IV - 输入 BST
给定一个二叉搜索树和一个目标结果，如果 BST 中存在两个元素且它们的和等于给定的目标结果，则返回 true。

#### 案例 1:
<pre>
<strong>输入:</strong>
    5
   / \
  3   6
 / \   \
2   4   7

Target = 9
<strong>输出:</strong> True
</pre>

#### 案例 2:
<pre>
<strong>输入:</strong>
    5
   / \
  3   6
 / \   \
2   4   7

Target = 28
<strong>输出:</strong> False
</pre>

## 题解 (Python)

### 1. 集合
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        def helper(root: TreeNode, k: int, vals: set) -> bool:
            if not root:
                return False

            if k - root.val in vals:
                return True

            vals.add(root.val)

            return helper(root.left, k, vals) or helper(root.right, k, vals)

        return helper(root, k, set())
```

### 2. 中序遍历
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        nodes = []
        curr = root
        vals = []

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()
            vals.append(curr.val)

            curr = curr.right

        l, r = 0, len(vals) - 1

        while l < r:
            if vals[l] + vals[r] < k:
                l += 1
            elif vals[l] + vals[r] > k:
                r -= 1
            else:
                return True

        return False
```
