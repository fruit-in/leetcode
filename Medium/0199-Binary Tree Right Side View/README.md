# 199. Binary Tree Right Side View
Given a binary tree, imagine yourself standing on the *right* side of it, return the values of the nodes you can see ordered from top to bottom.

#### Example:
<pre>
<strong>Input:</strong> [1,2,3,null,5,null,4]
<strong>Output:</strong> [1, 3, 4]
<strong>Explanation:</strong>
   1            <---
 /   \
2     3         <---
 \     \
  5     4       <---
</pre>

## Solutions (Python)

### 1. BFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def rightSideView(self, root: TreeNode) -> List[int]:
        if not root:
            return []

        ret = []
        curr_level = [root]

        while curr_level:
            next_level = []

            for node in curr_level:
                if node.left:
                    next_level.append(node.left)
                if node.right:
                    next_level.append(node.right)

            ret.append(curr_level[-1].val)
            curr_level = next_level

        return ret
```

### 2. DFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def rightSideView(self, root: TreeNode) -> List[int]:
        if not root:
            return []

        ret = []
        stack = [(root, 0)]

        while stack:
            curr, depth = stack.pop()

            if depth >= len(ret):
                ret.append(curr.val)

            if curr.left:
                stack.append((curr.left, depth + 1))
            if curr.right:
                stack.append((curr.right, depth + 1))

        return ret
```
