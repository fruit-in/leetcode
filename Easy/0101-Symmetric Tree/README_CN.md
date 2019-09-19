# 101. 对称二叉树
给定一个二叉树，检查它是否是镜像对称的。

例如，二叉树 ```[1,2,2,3,4,4,3]``` 是对称的。
```
    1
   / \
  2   2
 / \ / \
3  4 4  3
```

但是下面这个 ```[1,2,2,null,3,null,3]``` 则不是镜像对称的:
```
    1
   / \
  2   2
   \   \
   3    3
```

#### 说明:
如果你可以运用递归和迭代两种方法解决这个问题，会很加分。

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
    def isSymmetric(self, root: TreeNode) -> bool:
        def isMirror(root: TreeNode, toor: TreeNode) -> bool:
            if not root and not toor:
                return True
            if not root or not toor or root.val != toor.val:
                return False
            return isMirror(root.left, toor.right) and isMirror(root.right, toor.left)

        return isMirror(root, root)
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
