# 508. 出现次数最多的子树元素和
给你一个二叉树的根结点，请你找出出现次数最多的子树元素和。一个结点的「子树元素和」定义为以该结点为根的二叉树上所有结点的元素之和（包括结点本身）。

你需要返回出现次数最多的子树元素和。如果有多个元素出现的次数相同，返回所有出现次数最多的子树元素和（不限顺序）。

#### 示例 1:
输入:
```
  5
 /  \
2   -3
```
返回 [2, -3, 4]，所有的值均只出现一次，以任意顺序返回所有值。

#### 示例 2:
输入:
```
  5
 /  \
2   -5
```
返回 [2]，只有 2 出现两次，-5 只出现 1 次。

**提示:** 假设任意子树元素和均可以用 32 位有符号整数表示。

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findFrequentTreeSum(self, root: TreeNode) -> List[int]:
        def foo(root: TreeNode) -> int:
            sum = root.val
            sum += foo(root.left) if root.left else 0
            sum += foo(root.right) if root.right else 0

            freq[sum] = freq[sum] + 1 if sum in freq else 0

            return sum

        if not root:
            return []

        freq = {}
        foo(root)
        max_freq = max(freq.values())

        return [k for k, v in freq.items() if v == max_freq]
```
