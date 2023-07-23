# 513. Find Bottom Left Tree Value
Given a binary tree, find the leftmost value in the last row of the tree.

#### Example 1:
<pre>
<strong>Input:</strong>
    2
   / \
  1   3
<strong>Output:</strong>
1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
        1
       / \
      2   3
     /   / \
    4   5   6
       /
      7
<strong>Output:</strong>
7
</pre>

#### Note:
You may assume the tree (i.e., the given root node) is not **NULL**.

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
    def findBottomLeftValue(self, root: TreeNode) -> int:
        curr_level = [root]

        while curr_level:
            ret = curr_level[0].val
            next_level = [node.left for node in curr_level if node.left]
            next_level.extend(node.right for node in curr_level if node.right)
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
    def findBottomLeftValue(self, root: TreeNode) -> int:
        def dfs(root: TreeNode, depth: int) -> (int, int):
            ori_depth = depth
            val, depth = root.val, depth

            if root.left:
                val, depth = dfs(root.left, ori_depth + 1)
            if root.right:
                right_val, right_depth = dfs(root.right, ori_depth + 1)
                if right_depth > depth:
                    val, depth = right_val, right_depth

            return val, depth

        return dfs(root, 1)[0]
```
