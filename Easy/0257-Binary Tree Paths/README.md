# 257. Binary Tree Paths
Given a binary tree, return all root-to-leaf paths.

**Note:** A leaf is a node with no children.

#### Example:
<pre>
<strong>Input:</strong>

   1
 /   \
2     3
 \
  5

<strong>Output:</strong> ["1->2->5", "1->3"]
<strong>Explanation:</strong> All root-to-leaf paths are: 1->2->5, 1->3
</pre>

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
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []
        if not root.left and not root.right:
            return [str(root.val)]

        paths = self.binaryTreePaths(root.left)
        paths.extend(self.binaryTreePaths(root.right))

        return ["%d->" % root.val + path for path in paths]
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
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []

        paths = []
        nodes = [(root, str(root.val))]

        while nodes:
            curr, path = nodes.pop()

            if not curr.left and not curr.right:
                paths.append(path)
            if curr.left:
                nodes.append((curr.left, path + "->" + str(curr.left.val)))
            if curr.right:
                nodes.append((curr.right, path + "->" + str(curr.right.val)))

        return paths
```
