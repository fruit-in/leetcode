# 124. Binary Tree Maximum Path Sum
A **path** in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence **at most once**. Note that the path does not need to pass through the root.

The **path sum** of a path is the sum of the node's values in the path.

Given the `root` of a binary tree, return *the maximum **path sum** of any **non-empty** path*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg)
<pre>
<strong>Input:</strong> root = [-10,9,20,null,null,15,7]
<strong>Output:</strong> 42
<strong>Explanation:</strong> The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[1, 3 * 10<sup>4</sup>]</code>.
* `-1000 <= Node.val <= 1000`

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
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def dfs(root: Optional[TreeNode]) -> (int, int):
            if root is None:
                return (-1000, 0)

            lsubmax, lchildmax = dfs(root.left)
            rsubmax, rchildmax = dfs(root.right)
            submax = max(lsubmax, rsubmax, lchildmax + rchildmax + root.val)
            rootmax = max(0, root.val + max(lchildmax, rchildmax))

            return (submax, rootmax)

        return dfs(root)[0]
```
