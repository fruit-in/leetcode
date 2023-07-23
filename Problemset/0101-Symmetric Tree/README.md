# 101. Symmetric Tree
Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

For example, this binary tree <code>[1,2,2,3,4,4,3]</code> is symmetric:
```
    1
   / \
  2   2
 / \ / \
3  4 4  3
```

But the following <code>[1,2,2,null,3,null,3]</code> is not:
```
    1
   / \
  2   2
   \   \
   3    3
```

#### Note:
Bonus points if you could solve it both recursively and iteratively.

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
    def isSymmetric(self, root: TreeNode) -> bool:
        def isMirror(root: TreeNode, toor: TreeNode) -> bool:
            if not root and not toor:
                return True
            if not root or not toor or root.val != toor.val:
                return False
            return isMirror(root.left, toor.right) and isMirror(root.right, toor.left)

        return isMirror(root, root)
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
    def isSymmetric(self, root: TreeNode) -> bool:
        if not root:
            return True
        stack = [root.left, root.right]
        while stack:
            n1, n2 = stack.pop(), stack.pop()
            if not n1 and not n2:
                continue
            if not n1 or not n2 or n1.val != n2.val:
                return False
            stack.append(n1.left)
            stack.append(n2.right)
            stack.append(n1.right)
            stack.append(n2.left)
        return True
```
