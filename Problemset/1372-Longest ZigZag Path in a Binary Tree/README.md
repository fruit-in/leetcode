# 1372. Longest ZigZag Path in a Binary Tree
You are given the `root` of a binary tree.

A ZigZag path for a binary tree is defined as follow:

* Choose **any** node in the binary tree and a direction (right or left).
* If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
* Change the direction from right to left or from left to right.
* Repeat the second and third steps until you can't move in the tree.

Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).

Return *the longest **ZigZag** path contained in that tree*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/22/sample_1_1702.png)
<pre>
<strong>Input:</strong> root = [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Longest ZigZag path in blue nodes (right -> left -> right).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/01/22/sample_2_1702.png)
<pre>
<strong>Input:</strong> root = [1,1,1,null,1,null,null,1,1,null,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Longest ZigZag path in blue nodes (left -> right -> left -> right).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[1, 5 * 10<sup>4</sup>]</code>.
* `1 <= Node.val <= 100`

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
    def longestZigZag(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[0]

    def dfs(self, root: Optional[TreeNode]) -> (int, int, int):
        if root is None:
            return (0, -1, -1)

        left = self.dfs(root.left)
        right = self.dfs(root.right)
        maxpath = max(left[0], left[1], left[2] + 1,
                      right[0], right[1] + 1, right[2])

        return (maxpath, left[2] + 1, right[1] + 1)
```
