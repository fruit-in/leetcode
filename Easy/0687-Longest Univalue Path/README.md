# 687. Longest Univalue Path
Given a binary tree, find the length of the longest path where each node in the path has the same value. This path may or may not pass through the root.

The length of path between two nodes is represented by the number of edges between them.

#### Example 1:
<pre>
<strong>Input:</strong>
              5
             / \
            4   5
           / \   \
          1   1   5
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
              1
             / \
            4   5
           / \   \
          4   4   5
<strong>Output:</strong> 2
</pre>

**Note:** The given binary tree has not more than 10000 nodes. The height of the tree is not more than 1000.

## Solutions (Python)

### 1. DFS
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def longestUnivaluePath(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            longer, longest = 1, 0
            l = helper(root.left)
            r = helper(root.right)

            if root.left and root.val == root.left.val:
                longer = l[0] + 1
                longest += l[0]
            if root.right and root.val == root.right.val:
                longer = max(longer, r[0] + 1)
                longest += r[0]

            return (longer, max(l[1], r[1], longest))

        return helper(root)[1]
```
