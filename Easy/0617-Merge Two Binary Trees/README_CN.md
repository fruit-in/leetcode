# 617. 合并二叉树
给定两个二叉树，想象当你将它们中的一个覆盖到另一个上时，两个二叉树的一些节点便会重叠。

你需要将他们合并为一个新的二叉树。合并的规则是如果两个节点重叠，那么将他们的值相加作为节点合并后的新值，否则**不为** NULL 的节点将直接作为新二叉树的节点。

#### 示例 1:
<pre>
<strong>输入:</strong>
	Tree 1                     Tree 2
          1                         2
         / \                       / \
        3   2                     1   3
       /                           \   \
      5                             4   7
<strong>输出:</strong>
合并后的树:
	     3
	    / \
	   4   5
	  / \   \
	 5   4   7
</pre>

**注意:** 合并必须从两个树的根节点开始。

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
    def mergeTrees(self, t1: TreeNode, t2: TreeNode) -> TreeNode:
        if not t1:
            t1 = t2
        elif t1 and t2:
            t1.val += t2.val
            t1.left = self.mergeTrees(t1.left, t2.left)
            t1.right = self.mergeTrees(t1.right, t2.right)
        return t1
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
    def mergeTrees(self, t1: TreeNode, t2: TreeNode) -> TreeNode:
        if not t1:
            return t2
        if not t2:
            return t1
        stack = [(t1, t2)]
        while stack:
            n1, n2 = stack.pop()
            n1.val += n2.val
            if n1.left and n2.left:
                stack.append((n1.left, n2.left))
            elif not n1.left and n2.left:
                n1.left = n2.left
            if n1.right and n2.right:
                stack.append((n1.right, n2.right))
            elif not n1.right and n2.right:
                n1.right = n2.right
        return t1
```
