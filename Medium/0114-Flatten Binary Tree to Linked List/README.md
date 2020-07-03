# 114. Flatten Binary Tree to Linked List
Given a binary tree, flatten it to a linked list in-place.

For example, given the following tree:
```
    1
   / \
  2   5
 / \   \
3   4   6
```

The flattened tree should look like:
```
1
 \
  2
   \
    3
     \
      4
       \
        5
         \
          6
```

## Solutions (Python)

### 1. DFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def flatten(self, root: TreeNode) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        if not root:
            return None

        stack = [root]
        prev = TreeNode(left=root)

        while stack:
            curr = stack.pop()

            prev.left = None
            prev.right = curr

            if curr.right:
                stack.append(curr.right)
            if curr.left:
                stack.append(curr.left)

            prev = curr
```
