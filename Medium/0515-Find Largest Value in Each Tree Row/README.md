# 515. Find Largest Value in Each Tree Row
You need to find the largest value in each row of a binary tree.

#### Example:
<pre>
<strong>Input:</strong>
          1
         / \
        3   2
       / \   \
      5   3   9
<strong>Output:</strong> [1, 3, 9]
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
    def largestValues(self, root: TreeNode) -> List[int]:
        if not root:
            return []

        curr_level = [root]
        ret = []

        while curr_level:
            ret.append(max(node.val for node in curr_level))
            next_level = [node.left for node in curr_level if node.left]
            next_level.extend(node.right for node in curr_level if node.right)
            curr_level = next_level

        return ret
```
