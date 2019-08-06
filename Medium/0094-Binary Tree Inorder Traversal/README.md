# 94. Binary Tree Inorder Traversal
Given a binary tree, return the *inorder* traversal of its nodes' values.

#### Example:
<pre>
<strong>Input:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>Output:</strong> [1,3,2]
</pre>

<strong>Follow up:</strong> Recursive solution is trivial, could you do it iteratively?

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
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return self.inorderTraversal(root.left) + \
            [root.val] + self.inorderTraversal(root.right)
```

### 2. Stack
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        vals = []
        nodes = []
        node = root
        while node or nodes:
            while node:
                nodes.append(node)
                node = node.left
            node = nodes.pop()
            vals.append(node.val)
            node = node.right
        return vals
```
