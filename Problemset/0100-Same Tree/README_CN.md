# 100. 相同的树
给定两个二叉树，编写一个函数来检验它们是否相同。

如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。

#### 示例 1:
<pre>
<strong>输入:</strong>       1         1
          / \       / \
         2   3     2   3


        [1,2,3],   [1,2,3]

<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>       1         1
          /           \
         2             2


        [1,2],     [1,null,2]

<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong>       1         1
          / \       / \
         2   1     1   2


        [1,2,1],   [1,1,2]

<strong>输出:</strong> false
</pre>

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
    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        if not p and not q:
            return True
        if not p or not q or p.val != q.val:
            return False
        return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
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
    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        stackp, stackq = [p], [q]
        while stackp:
            nodep, nodeq = stackp.pop(), stackq.pop()
            if not nodep and not nodeq:
                continue
            if not nodep or not nodeq or nodep.val != nodeq.val:
                return False
            stackp.append(nodep.left)
            stackq.append(nodeq.left)
            stackp.append(nodep.right)
            stackq.append(nodeq.right)
        return True
```
