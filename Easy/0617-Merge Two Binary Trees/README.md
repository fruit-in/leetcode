# 617. Merge Two Binary Trees
Given two binary trees and imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not.

You need to merge them into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of new tree.

#### Example 1:
<pre>
<strong>Input:</strong>
	Tree 1                     Tree 2
          1                         2
         / \                       / \
        3   2                     1   3
       /                           \   \
      5                             4   7
<strong>Output:</strong>
Merged tree:
	     3
	    / \
	   4   5
	  / \   \
	 5   4   7
</pre>

**Note:** The merging process must start from the root nodes of both trees.

## Solutions (Python)

### 1. Recursion
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

### 2. Iteration
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
