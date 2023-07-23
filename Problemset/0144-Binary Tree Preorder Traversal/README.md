# 144. Binary Tree Preorder Traversal
Given a binary tree, return the *preorder* traversal of its nodes' values.

#### Example:
<pre>
<strong>Input:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>Output:</strong> [1,2,3]
</pre>

**Follow up:** Recursive solution is trivial, could you do it iteratively?

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
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return [root.val] + \
            self.preorderTraversal(root.left) + \
            self.preorderTraversal(root.right)
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
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        vals = []
        nodes = [root]
        while nodes:
            node = nodes.pop()
            vals.append(node.val)
            if node.right:
                nodes.append(node.right)
            if node.left:
                nodes.append(node.left)
        return vals
```
