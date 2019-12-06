# 572. 另一个树的子树
给定两个非空二叉树 **s** 和 **t**，检验 **s** 中是否包含和 **t** 具有相同结构和节点值的子树。**s** 的一个子树包括 **s** 的一个节点和这个节点的所有子孙。**s** 也可以看做它自身的一棵子树。

#### 示例 1:
给定的树 s:
```
     3
    / \
   4   5
  / \
 1   2
```
给定的树 t:
```
   4
  / \
 1   2
```
返回 **true**，因为 t 与 s 的一个子树拥有相同的结构和节点值。

#### 示例 2:
给定的树 s:
```
     3
    / \
   4   5
  / \
 1   2
    /
   0
```
给定的树 t:
```
   4
  / \
 1   2
```
返回 **false**。

## 题解 (Python)

### 1. 比较节点
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        def isEqual(s: TreeNode, t: TreeNode) -> bool:
            if s and t and s.val == t.val:
                return isEqual(s.left, t.left) and isEqual(s.right, t.right)
            elif not s and not t:
                return True
            else:
                return False


        return s and (isEqual(s, t) or self.isSubtree(s.left, t) or self.isSubtree(s.right, t))
```

### 2. 转换为字符串
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        def toString(root: TreeNode) -> str:
            if not root:
                return 'N'
            return "~%d%s%s" % (root.val, toString(root.left), toString(root.right))


        return toString(t) in toString(s)
```
