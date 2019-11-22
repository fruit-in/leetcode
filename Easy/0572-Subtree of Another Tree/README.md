# 572. Subtree of Another Tree
Given two non-empty binary trees **s** and **t**, check whether tree **t** has exactly the same structure and node values with a subtree of **s**. A subtree of **s** is a tree consists of a node in **s** and all of this node's descendants. The tree **s** could also be considered as a subtree of itself.

#### Example 1:
Given tree s:
```
     3
    / \
   4   5
  / \
 1   2
```
Given tree t:
```
   4
  / \
 1   2
```
Return **true**, because t has the same structure and node values with a subtree of s.

#### Example 2:
Given tree s:
```
     3
    / \
   4   5
  / \
 1   2
    /
   0
```
Given tree t:
```
   4
  / \
 1   2
```
Return **false**.

## Solutions (Python)

### 1. Compare Nodes
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

### 2. Convert to String
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
