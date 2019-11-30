# 538. 把二叉搜索树转换为累加树
给定一个二叉搜索树（Binary Search Tree），把它转换成为累加树（Greater Tree)，使得每个节点的值是原来的节点值加上所有大于它的节点值之和。

#### 例如:
<pre>
<strong>输入:</strong> 二叉搜索树:
              5
            /   \
           2     13
<strong>输出:</strong> 转换为累加树:
             18
            /   \
          20     13
</pre>

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
    def convertBST(self, root: TreeNode) -> TreeNode:
        nodes = []
        curr = root
        sum = 0

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
```
