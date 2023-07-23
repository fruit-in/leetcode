# 113. Path Sum II
Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.

**Note:** A leaf is a node with no children.

#### Example:
Given the below binary tree and `sum = 22`,
<pre>
      <b>5</b>
     <b>/ \</b>
    <b>4   8</b>
   <b>/</b>   / <b>\</b>
  <b>11</b>  13  <b>4</b>
 /  <b>\    /</b> \
7    <b>2  5</b>   1
</pre>

Return:
```
[
   [5,4,11,2],
   [5,8,4,5]
]
```

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        if not root:
            return []

        if not (root.left or root.right) and root.val == sum:
            return [[root.val]]

        paths = self.pathSum(root.left, sum - root.val)
        paths.extend(self.pathSum(root.right, sum - root.val))

        for path in paths:
            path.insert(0, root.val)

        return paths
```
