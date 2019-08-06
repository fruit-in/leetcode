# 145. Binary Tree Postorder Traversal
Given a binary tree, return the *postorder* traversal of its nodes' values.

#### Example:
<pre>
<strong>Input:</strong> [1,null,2,3]
   1
    \
     2
    /
   3

<strong>Output:</strong> [3,2,1]
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
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        if not root:
            return []
        return self.postorderTraversal(root.left) + \
            self.postorderTraversal(root.right) + [root.val]
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
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        nset = set()
        vals = []
        nodes = [None]
        node = root
        
        while nodes and node:
            while node.left and node.left not in nset:
                nodes.append(node)
                node = node.left
            if node.right and node.right not in nset:
                nodes.append(node)
                node = node.right
            else:
                vals.append(node.val)
                nset.add(node)
                node = nodes.pop()
        return vals
```
