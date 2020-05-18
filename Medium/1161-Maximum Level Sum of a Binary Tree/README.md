# 1161. Maximum Level Sum of a Binary Tree
Given the `root` of a binary tree, the level of its root is `1`, the level of its children is `2`, and so on.

Return the **smallest** level `X` such that the sum of all the values of nodes at level `X` is **maximal**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/05/03/capture.JPG)
<pre>
<strong>Input:</strong> [1,7,0,7,-8,null,null]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Level 1 sum = 1.
Level 2 sum = 7 + 0 = 7.
Level 3 sum = 7 + -8 = -1.
So we return the level with the maximum sum which is level 2.
</pre>

#### Note:
1. The number of nodes in the given tree is between `1` and `10^4`.
2. `-10^5 <= node.val <= 10^5`

## Solutions (Python)

### 1. BFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxLevelSum(self, root: TreeNode) -> int:
        curr_level = [root]
        max_sum = root.val
        x = 1
        ret = 1

        while curr_level:
            curr_sum = sum(n.val for n in curr_level)
            if curr_sum > max_sum:
                max_sum = curr_sum
                ret = x

            curr_level = [c for n in curr_level for c in [n.left, n.right] if c]
            x += 1

        return ret
```
