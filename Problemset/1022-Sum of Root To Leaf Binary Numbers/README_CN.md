# 1022. 从根到叶的二进制数之和
给出一棵二叉树，其上每个结点的值都是 ```0``` 或 ```1``` 。每一条从根到叶的路径都代表一个从最高有效位开始的二进制数。例如，如果路径为 ```0 -> 1 -> 1 -> 0 -> 1```，那么它表示二进制数 ```01101```，也就是 ```13``` 。

对树上的每一片叶子，我们都要找出从根到该叶子的路径所表示的数字。

以 **```10^9 + 7```** 为**模**，返回这些数字之和。

#### 示例:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/04/05/sum-of-root-to-leaf-binary-numbers.png)
<pre>
<strong>输入:</strong> [1,0,1,0,1,0,1]
<strong>输入:</strong> 22
<strong>解释:</strong> (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
</pre>

#### 提示:
1. 树中的结点数介于 ```1``` 和 ```1000``` 之间。
2. node.val 为 ```0``` 或 ```1``` 。

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
    def sumRootToLeaf(self, root: TreeNode) -> int:
        def helper(root: TreeNode, n: int) -> int:
            n = (n << 1) + root.val

            if root.left and root.right:
                return helper(root.left, n) + helper(root.right, n)
            elif root.left:
                return helper(root.left, n)
            elif root.right:
                return helper(root.right, n)
            else:
                return n

        return helper(root, 0)
```

### 2. 迭代
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        nodes = [(root, 0)]
        nums = []

        while nodes:
            cur, num = nodes.pop()

            if not cur.left and not cur.right:
                nums.append((num << 1) + cur.val)
            if cur.left:
                nodes.append((cur.left, (num << 1) + cur.val))
            if cur.right:
                nodes.append((cur.right, (num << 1) + cur.val))

        return sum(nums)
```
